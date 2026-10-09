-- Schema inicial do catálogo. Execute com SQLite 3.
-- Produtos de fontes distintas ficam separados até existir regra segura de canonicalização.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS stores (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    official_url TEXT,
    city TEXT NOT NULL DEFAULT 'Londrina',
    state TEXT NOT NULL DEFAULT 'PR',
    address TEXT,
    phone_public TEXT,
    verification_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (verification_status IN ('confirmed', 'pending', 'stale')),
    source_url TEXT,
    verified_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sources (
    id TEXT PRIMARY KEY,
    organization_name TEXT NOT NULL,
    official_url TEXT NOT NULL,
    product_listing_url TEXT,
    source_type TEXT NOT NULL
        CHECK (source_type IN ('site', 'api', 'feed', 'authorized_file', 'manual')),
    geographic_scope TEXT NOT NULL DEFAULT 'unknown',
    access_method TEXT NOT NULL,
    permission_status TEXT NOT NULL DEFAULT 'discovered'
        CHECK (permission_status IN ('discovered', 'under_review', 'approved', 'restricted', 'blocked', 'inactive')),
    permission_evidence_url TEXT,
    permitted_fields TEXT,
    media_permission TEXT NOT NULL DEFAULT 'not_assessed',
    expected_refresh TEXT,
    last_checked_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS import_batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id TEXT NOT NULL REFERENCES sources(id),
    file_name TEXT NOT NULL,
    started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TEXT,
    rows_read INTEGER NOT NULL DEFAULT 0,
    accepted_rows INTEGER NOT NULL DEFAULT 0,
    inserted_rows INTEGER NOT NULL DEFAULT 0,
    updated_rows INTEGER NOT NULL DEFAULT 0,
    rejected_rows INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'running'
        CHECK (status IN ('running', 'completed', 'failed', 'dry_run')),
    report_json TEXT
);

CREATE TABLE IF NOT EXISTS source_products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id TEXT NOT NULL REFERENCES sources(id),
    source_product_id TEXT,
    product_name TEXT NOT NULL,
    brand TEXT,
    category_path TEXT NOT NULL,
    manufacturer_code TEXT,
    gtin TEXT,
    variant TEXT,
    sale_unit TEXT NOT NULL,
    package_quantity TEXT,
    package_unit TEXT,
    short_description TEXT,
    product_url TEXT NOT NULL,
    image_url TEXT,
    observed_at TEXT NOT NULL,
    reuse_permission TEXT NOT NULL CHECK (reuse_permission = 'authorized'),
    content_hash TEXT NOT NULL,
    first_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_import_batch_id INTEGER REFERENCES import_batches(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_source_products_source_code
ON source_products(source_id, source_product_id)
WHERE source_product_id IS NOT NULL AND source_product_id <> '';

CREATE UNIQUE INDEX IF NOT EXISTS ux_source_products_source_url
ON source_products(source_id, product_url)
WHERE source_product_id IS NULL OR source_product_id = '';

CREATE INDEX IF NOT EXISTS ix_source_products_name ON source_products(product_name);
CREATE INDEX IF NOT EXISTS ix_source_products_category ON source_products(category_path);
CREATE INDEX IF NOT EXISTS ix_source_products_brand ON source_products(brand);
CREATE INDEX IF NOT EXISTS ix_source_products_gtin ON source_products(gtin);

CREATE TABLE IF NOT EXISTS offers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id TEXT NOT NULL REFERENCES sources(id),
    store_id TEXT NOT NULL REFERENCES stores(id),
    source_offer_id TEXT,
    source_product_id INTEGER REFERENCES source_products(id),
    price_amount NUMERIC CHECK (price_amount IS NULL OR price_amount >= 0),
    currency TEXT NOT NULL DEFAULT 'BRL',
    price_type TEXT NOT NULL DEFAULT 'unknown'
        CHECK (price_type IN ('regular', 'sale', 'cash', 'installment', 'conditional', 'unknown')),
    availability TEXT NOT NULL DEFAULT 'unknown'
        CHECK (availability IN ('in_stock', 'out_of_stock', 'unknown', 'not_applicable')),
    fulfillment TEXT NOT NULL DEFAULT 'unknown'
        CHECK (fulfillment IN ('online', 'store', 'pickup', 'delivery', 'unknown')),
    conditions TEXT,
    offer_url TEXT NOT NULL,
    observed_at TEXT NOT NULL,
    expires_at TEXT,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('current', 'expired', 'unavailable', 'pending')),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_offers_source_offer_id
ON offers(source_id, source_offer_id)
WHERE source_offer_id IS NOT NULL AND source_offer_id <> '';

CREATE UNIQUE INDEX IF NOT EXISTS ux_offers_source_store_url
ON offers(source_id, store_id, offer_url)
WHERE source_offer_id IS NULL OR source_offer_id = '';

CREATE INDEX IF NOT EXISTS ix_offers_product ON offers(source_product_id);
CREATE INDEX IF NOT EXISTS ix_offers_store ON offers(store_id);
CREATE INDEX IF NOT EXISTS ix_offers_observed ON offers(observed_at);
CREATE INDEX IF NOT EXISTS ix_offers_price ON offers(price_amount);

CREATE TABLE IF NOT EXISTS import_errors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id INTEGER NOT NULL REFERENCES import_batches(id) ON DELETE CASCADE,
    row_number INTEGER,
    error_code TEXT NOT NULL,
    message TEXT NOT NULL,
    raw_row_json TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
