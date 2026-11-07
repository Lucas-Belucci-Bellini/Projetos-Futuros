use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rusqlite::{params_from_iter, types::Value, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    q: Option<String>,
    category_id: Option<String>,
    brand: Option<String>,
    store_id: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    page: Option<u32>,
    page_size: Option<u32>,
    sort: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductPage {
    data: Vec<ProductSummary>,
    pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    page: u32,
    page_size: u32,
    total_items: u64,
    total_pages: u64,
}

#[derive(Debug, Serialize)]
pub struct ProductSummary {
    id: String,
    name: String,
    brand: Option<String>,
    category: Option<CategoryRef>,
    unit: Option<String>,
    offers_count: u64,
}

#[derive(Debug, Serialize)]
struct CategoryRef {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryList {
    data: Vec<Category>,
}

#[derive(Debug, Serialize)]
struct Category {
    id: String,
    name: String,
    parent_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductDetail {
    id: String,
    name: String,
    brand: Option<String>,
    category: Option<CategoryRef>,
    unit: Option<String>,
    offers_count: usize,
    variant: Option<String>,
    model_code: Option<String>,
    offers: Vec<Offer>,
}

#[derive(Debug, Serialize)]
struct Offer {
    id: String,
    store: String,
    price: Option<String>,
    currency: String,
    price_type: String,
    fulfillment: String,
    availability: String,
    conditions: Option<String>,
    offer_url: String,
    observed_at: String,
}

#[derive(Debug)]
pub struct CatalogError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl CatalogError {
    fn bad_query() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_QUERY",
            message: "Um ou mais parâmetros de pesquisa são inválidos.",
        }
    }

    fn unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "SERVICE_UNAVAILABLE",
            message: "O catálogo está temporariamente indisponível.",
        }
    }

    fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "NOT_FOUND",
            message: "Produto não encontrado.",
        }
    }
}

impl IntoResponse for CatalogError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(serde_json::json!({
                "error": {
                    "code": self.code,
                    "message": self.message,
                }
            })),
        )
            .into_response()
    }
}

#[derive(Default)]
struct Filters {
    clauses: Vec<String>,
    values: Vec<Value>,
}

impl Filters {
    fn new() -> Self {
        Self {
            clauses: vec![
                "sp.reuse_permission = 'authorized'".into(),
                "s.permission_status = 'approved'".into(),
            ],
            values: Vec::new(),
        }
    }

    fn push(&mut self, clause: &str, value: Value) {
        self.clauses.push(clause.to_owned());
        self.values.push(value);
    }

    fn where_sql(&self) -> String {
        self.clauses.join(" AND ")
    }
}

pub async fn list_products(
    State(state): State<AppState>,
    Query(query): Query<ProductQuery>,
) -> Result<Json<ProductPage>, CatalogError> {
    let normalized = normalize_query(query)?;
    let database_path = state.database_path.clone();
    tokio::task::spawn_blocking(move || search_products(&database_path, normalized))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de busca do catálogo falhou");
            CatalogError::unavailable()
        })?
        .map(Json)
}

pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<CategoryList>, CatalogError> {
    let database_path = state.database_path.clone();
    let categories = tokio::task::spawn_blocking(move || {
        let connection = open_database(&database_path)?;
        let mut statement = connection
            .prepare(
                "SELECT DISTINCT sp.category_path
                 FROM source_products sp
                 JOIN sources s ON s.id = sp.source_id
                 WHERE sp.reuse_permission = 'authorized'
                   AND s.permission_status = 'approved'
                 ORDER BY sp.category_path COLLATE NOCASE",
            )
            .map_err(database_error)?;
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(database_error)?;
        rows.map(|row| {
            let path = row.map_err(database_error)?;
            let name = path.rsplit(" > ").next().unwrap_or(&path).to_owned();
            Ok(Category {
                id: path,
                name,
                parent_id: None,
            })
        })
        .collect::<Result<Vec<_>, CatalogError>>()
    })
    .await
    .map_err(|error| {
        tracing::error!(%error, "Tarefa de categorias falhou");
        CatalogError::unavailable()
    })??;

    Ok(Json(CategoryList { data: categories }))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProductDetail>, CatalogError> {
    if id.is_empty() || id.len() > 64 || !id.chars().all(|character| character.is_ascii_digit()) {
        return Err(CatalogError::not_found());
    }
    let database_path = state.database_path.clone();
    let product = tokio::task::spawn_blocking(move || product_detail(&database_path, id))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de detalhe do produto falhou");
            CatalogError::unavailable()
        })??;
    Ok(Json(product))
}

fn normalize_query(query: ProductQuery) -> Result<NormalizedQuery, CatalogError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(24);
    if page == 0 || !(1..=100).contains(&page_size) {
        return Err(CatalogError::bad_query());
    }

    let search = query.q.map(|value| value.trim().to_owned());
    if search.as_ref().is_some_and(|value| value.len() > 100) {
        return Err(CatalogError::bad_query());
    }
    if [&query.category_id, &query.brand, &query.store_id]
        .into_iter()
        .flatten()
        .any(|value| value.len() > 120)
    {
        return Err(CatalogError::bad_query());
    }
    for value in [&query.min_price, &query.max_price].into_iter().flatten() {
        if !value.is_finite() || *value < 0.0 {
            return Err(CatalogError::bad_query());
        }
    }
    if matches!((query.min_price, query.max_price), (Some(min), Some(max)) if min > max) {
        return Err(CatalogError::bad_query());
    }
    let sort = query.sort.as_deref().unwrap_or("relevance");
    if !matches!(sort, "relevance" | "name_asc" | "price_asc" | "price_desc") {
        return Err(CatalogError::bad_query());
    }

    Ok(NormalizedQuery {
        search: search.filter(|value| !value.is_empty()),
        category_id: query.category_id,
        brand: query.brand,
        store_id: query.store_id,
        min_price: query.min_price,
        max_price: query.max_price,
        page,
        page_size,
        sort: sort.to_owned(),
    })
}

struct NormalizedQuery {
    search: Option<String>,
    category_id: Option<String>,
    brand: Option<String>,
    store_id: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    page: u32,
    page_size: u32,
    sort: String,
}

fn search_products(
    database_path: &std::path::Path,
    query: NormalizedQuery,
) -> Result<ProductPage, CatalogError> {
    let connection = open_database(database_path)?;
    let mut filters = Filters::new();
    let relevance_search = query.search.clone();

    if let Some(search) = query.search {
        filters.clauses.push(
            "(instr(lower(sp.product_name), lower(?)) > 0
              OR instr(lower(COALESCE(sp.brand, '')), lower(?)) > 0
              OR instr(lower(COALESCE(sp.manufacturer_code, '')), lower(?)) > 0
              OR instr(lower(COALESCE(sp.gtin, '')), lower(?)) > 0)"
                .to_owned(),
        );
        for _ in 0..4 {
            filters.values.push(Value::Text(search.clone()));
        }
    }
    if let Some(category_id) = query.category_id {
        filters.push("sp.category_path = ?", Value::Text(category_id));
    }
    if let Some(brand) = query.brand {
        filters.push(
            "lower(COALESCE(sp.brand, '')) = lower(?)",
            Value::Text(brand),
        );
    }
    if let Some(store_id) = query.store_id.as_ref() {
        filters.push(
            "EXISTS (
                SELECT 1 FROM offers o
                JOIN sources offer_source ON offer_source.id = o.source_id
                JOIN stores st ON st.id = o.store_id
                WHERE o.source_product_id = sp.id
                  AND o.status = 'current'
                  AND offer_source.permission_status = 'approved'
                  AND st.verification_status = 'confirmed'
                  AND st.id = ?
            )",
            Value::Text(store_id.clone()),
        );
    }
    if query.min_price.is_some() || query.max_price.is_some() {
        let mut price_conditions = vec![
            "o.source_product_id = sp.id",
            "o.status = 'current'",
            "offer_source.permission_status = 'approved'",
            "st.verification_status = 'confirmed'",
            "o.price_amount IS NOT NULL",
        ];
        let mut price_values = Vec::new();
        if let Some(store_id) = query.store_id.as_ref() {
            price_conditions.push("o.store_id = ?");
            price_values.push(Value::Text(store_id.clone()));
        }
        if let Some(min_price) = query.min_price {
            price_conditions.push("o.price_amount >= ?");
            price_values.push(Value::Real(min_price));
        }
        if let Some(max_price) = query.max_price {
            price_conditions.push("o.price_amount <= ?");
            price_values.push(Value::Real(max_price));
        }
        filters.clauses.push(format!(
            "EXISTS (
                    SELECT 1 FROM offers o
                    JOIN sources offer_source ON offer_source.id = o.source_id
                    JOIN stores st ON st.id = o.store_id
                    WHERE {}
                )",
            price_conditions.join(" AND ")
        ));
        filters.values.extend(price_values);
    }

    let where_clause = filters.where_sql();
    let count_sql = format!(
        "SELECT COUNT(*)
         FROM source_products sp
         JOIN sources s ON s.id = sp.source_id
         WHERE {where_clause}"
    );
    let total_items: u64 = connection
        .query_row(
            &count_sql,
            params_from_iter(filters.values.clone()),
            |row| row.get(0),
        )
        .map_err(database_error)?;

    let order = match (
        query.sort.as_str(),
        relevance_search.is_some(),
        query.store_id.is_some(),
    ) {
        ("relevance", true, _) => {
            "CASE WHEN lower(sp.product_name) = lower(?) THEN 0
                  WHEN instr(lower(sp.product_name), lower(?)) > 0 THEN 1
                  ELSE 2 END, sp.product_name COLLATE NOCASE ASC"
        }
        ("name_asc", _, _) => "sp.product_name COLLATE NOCASE ASC",
        ("price_asc", _, true) => {
            "COALESCE(MIN(CASE WHEN o.status = 'current'
                        AND offer_source.permission_status = 'approved'
                       AND st.verification_status = 'confirmed'
                         AND o.store_id = ?
                       THEN o.price_amount END), 1.0e308) ASC,
             sp.product_name COLLATE NOCASE ASC"
        }
        ("price_asc", _, false) => {
            "COALESCE(MIN(CASE WHEN o.status = 'current'
                         AND offer_source.permission_status = 'approved'
                        AND st.verification_status = 'confirmed'
                       THEN o.price_amount END), 1.0e308) ASC,
             sp.product_name COLLATE NOCASE ASC"
        }
        ("price_desc", _, true) => {
            "COALESCE(MIN(CASE WHEN o.status = 'current'
                        AND offer_source.permission_status = 'approved'
                       AND st.verification_status = 'confirmed'
                         AND o.store_id = ?
                       THEN o.price_amount END), -1) DESC,
             sp.product_name COLLATE NOCASE ASC"
        }
        ("price_desc", _, false) => {
            "COALESCE(MIN(CASE WHEN o.status = 'current'
                        AND offer_source.permission_status = 'approved'
                       AND st.verification_status = 'confirmed'
                       THEN o.price_amount END), -1) DESC,
             sp.product_name COLLATE NOCASE ASC"
        }
        _ => "sp.product_name COLLATE NOCASE ASC",
    };
    let page_sql = format!(
        "SELECT CAST(sp.id AS TEXT), sp.product_name, sp.brand, sp.category_path,
                sp.sale_unit, COUNT(DISTINCT CASE
                    WHEN o.status = 'current'
                     AND offer_source.permission_status = 'approved'
                     AND st.verification_status = 'confirmed'
                    THEN o.id END) AS offers_count
         FROM source_products sp
         JOIN sources s ON s.id = sp.source_id
         LEFT JOIN offers o ON o.source_product_id = sp.id
         LEFT JOIN sources offer_source ON offer_source.id = o.source_id
         LEFT JOIN stores st ON st.id = o.store_id
         WHERE {where_clause}
         GROUP BY sp.id
         ORDER BY {order}
         LIMIT ? OFFSET ?"
    );
    let offset = u64::from(query.page - 1) * u64::from(query.page_size);
    let mut page_values = filters.values;
    if query.sort == "relevance" {
        if let Some(search) = relevance_search {
            page_values.push(Value::Text(search.clone()));
            page_values.push(Value::Text(search));
        }
    }
    if matches!(query.sort.as_str(), "price_asc" | "price_desc") {
        if let Some(store_id) = query.store_id {
            if order.contains("o.store_id = ?") {
                page_values.push(Value::Text(store_id));
            }
        }
    }
    page_values.push(Value::Integer(i64::from(query.page_size)));
    page_values.push(Value::Integer(offset.min(i64::MAX as u64) as i64));
    let mut statement = connection.prepare(&page_sql).map_err(database_error)?;
    let rows = statement
        .query_map(params_from_iter(page_values), |row| {
            let category_path: Option<String> = row.get(3)?;
            let category = category_path.map(|path| CategoryRef {
                name: path.rsplit(" > ").next().unwrap_or(&path).to_owned(),
                id: path,
            });
            Ok(ProductSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                brand: row.get(2)?,
                category,
                unit: row.get(4)?,
                offers_count: row.get(5)?,
            })
        })
        .map_err(database_error)?;
    let data = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(database_error)?;

    let total_pages = if total_items == 0 {
        0
    } else {
        total_items.div_ceil(u64::from(query.page_size))
    };
    Ok(ProductPage {
        data,
        pagination: Pagination {
            page: query.page,
            page_size: query.page_size,
            total_items,
            total_pages,
        },
    })
}

fn product_detail(
    database_path: &std::path::Path,
    id: String,
) -> Result<ProductDetail, CatalogError> {
    let connection = open_database(database_path)?;
    let product = connection
        .query_row(
            "SELECT CAST(sp.id AS TEXT), sp.product_name, sp.brand, sp.category_path,
                    sp.sale_unit, sp.variant, sp.manufacturer_code
             FROM source_products sp
             JOIN sources s ON s.id = sp.source_id
             WHERE sp.id = ?
               AND sp.reuse_permission = 'authorized'
               AND s.permission_status = 'approved'",
            [&id],
            |row| {
                let category_path: Option<String> = row.get(3)?;
                let category = category_path.map(|path| CategoryRef {
                    name: path.rsplit(" > ").next().unwrap_or(&path).to_owned(),
                    id: path,
                });
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    category,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            },
        )
        .optional()
        .map_err(database_error)?
        .ok_or_else(CatalogError::not_found)?;

    let mut statement = connection
        .prepare(
            "SELECT CAST(o.id AS TEXT), st.name, CAST(o.price_amount AS TEXT), o.currency,
                    o.price_type, o.fulfillment, o.availability, o.conditions,
                    o.offer_url, o.observed_at
             FROM offers o
             JOIN sources s ON s.id = o.source_id
             JOIN stores st ON st.id = o.store_id
             WHERE o.source_product_id = ?
               AND o.status = 'current'
               AND s.permission_status = 'approved'
               AND st.verification_status = 'confirmed'
             ORDER BY o.price_amount IS NULL, o.price_amount ASC, o.observed_at DESC",
        )
        .map_err(database_error)?;
    let rows = statement
        .query_map([&id], |row| {
            Ok(Offer {
                id: row.get(0)?,
                store: row.get(1)?,
                price: row.get(2)?,
                currency: row.get(3)?,
                price_type: row.get(4)?,
                fulfillment: row.get(5)?,
                availability: row.get(6)?,
                conditions: row.get(7)?,
                offer_url: row.get(8)?,
                observed_at: row.get(9)?,
            })
        })
        .map_err(database_error)?;
    let offers = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(database_error)?;
    Ok(ProductDetail {
        id: product.0,
        name: product.1,
        brand: product.2,
        category: product.3,
        unit: product.4,
        offers_count: offers.len(),
        variant: product.5,
        model_code: product.6,
        offers,
    })
}

fn open_database(path: &std::path::Path) -> Result<Connection, CatalogError> {
    let connection = Connection::open(path).map_err(database_error)?;
    connection
        .busy_timeout(std::time::Duration::from_secs(3))
        .map_err(database_error)?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(database_error)?;
    Ok(connection)
}

fn database_error(error: rusqlite::Error) -> CatalogError {
    tracing::error!(%error, "Erro ao consultar o banco do catálogo");
    CatalogError::unavailable()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn validates_query_bounds_and_sort() {
        let valid = ProductQuery {
            q: Some(" cimento ".into()),
            category_id: None,
            brand: None,
            store_id: None,
            min_price: Some(0.0),
            max_price: Some(100.0),
            page: Some(1),
            page_size: Some(24),
            sort: Some("price_asc".into()),
        };
        assert!(normalize_query(valid).is_ok());

        let invalid = ProductQuery {
            q: None,
            category_id: None,
            brand: None,
            store_id: None,
            min_price: Some(101.0),
            max_price: Some(100.0),
            page: Some(1),
            page_size: Some(24),
            sort: None,
        };
        assert!(normalize_query(invalid).is_err());
    }

    #[test]
    fn search_excludes_unapproved_sources_and_counts_confirmed_offers() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "catalogo-search-test-{}-{nonce}.sqlite3",
            std::process::id()
        ));
        let connection = Connection::open(&path).unwrap();
        connection
            .execute_batch(include_str!("../../../database/001_initial_schema.sql"))
            .unwrap();
        connection
            .execute_batch(
                "INSERT INTO sources (
                    id, organization_name, official_url, source_type, access_method,
                    permission_status
                 ) VALUES ('approved', 'Fonte', 'https://example.test', 'authorized_file',
                           'arquivo autorizado', 'approved');
                 INSERT INTO stores (id, name, verification_status)
                 VALUES ('store', 'Loja verificada', 'confirmed');
                 INSERT INTO source_products (
                    source_id, product_name, category_path, sale_unit, product_url,
                    observed_at, reuse_permission, content_hash
                 ) VALUES ('approved', 'Cimento CP II', 'Materiais básicos > Cimento',
                           'saco', 'https://example.test/cimento', '2026-01-01T10:00:00Z',
                           'authorized', 'hash');
                 INSERT INTO offers (
                    source_id, store_id, source_product_id, price_amount, offer_url,
                    observed_at, status
                 ) VALUES ('approved', 'store', 1, 50.0, 'https://example.test/oferta',
                           '2026-01-01T10:00:00Z', 'current');
                 INSERT INTO sources (
                    id, organization_name, official_url, source_type, access_method,
                    permission_status
                 ) VALUES ('pending', 'Outra fonte', 'https://pending.test', 'manual',
                           'revisão', 'under_review');
                 INSERT INTO source_products (
                    source_id, product_name, category_path, sale_unit, product_url,
                    observed_at, reuse_permission, content_hash
                 ) VALUES ('pending', 'Cimento não aprovado', 'Materiais básicos > Cimento',
                           'saco', 'https://pending.test/cimento', '2026-01-01T10:00:00Z',
                           'authorized', 'hash-2');",
            )
            .unwrap();
        drop(connection);

        let page = search_products(
            &path,
            NormalizedQuery {
                search: Some("cimento".into()),
                category_id: None,
                brand: None,
                store_id: None,
                min_price: Some(40.0),
                max_price: Some(60.0),
                page: 1,
                page_size: 24,
                sort: "price_asc".into(),
            },
        )
        .unwrap();
        assert_eq!(page.pagination.total_items, 1);
        assert_eq!(page.data[0].name, "Cimento CP II");
        assert_eq!(page.data[0].offers_count, 1);

        let relevance_page = search_products(
            &path,
            NormalizedQuery {
                search: Some("cimento".into()),
                category_id: None,
                brand: None,
                store_id: None,
                min_price: None,
                max_price: None,
                page: 1,
                page_size: 24,
                sort: "relevance".into(),
            },
        )
        .unwrap();
        assert_eq!(relevance_page.data[0].name, "Cimento CP II");

        std::fs::remove_file(path).unwrap();
    }
}
