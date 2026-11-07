import { requestJson, isRecord } from "../../api/http";
import {
  isCategory,
  isHealthResponse,
  isProductDetail,
  isProductPage,
  type Category,
  type HealthResponse,
  type ProductDetail,
  type ProductSearch,
  type ProductPage,
} from "./types";

export async function getHealth(signal: AbortSignal): Promise<HealthResponse> {
  const body = await requestJson("/api/v1/health", "API indisponível", { signal });
  if (!isHealthResponse(body)) {
    throw new Error("A resposta de saúde da API é inválida.");
  }
  return body;
}

export async function getCategories(signal: AbortSignal): Promise<Category[]> {
  const body = await requestJson(
    "/api/v1/categories",
    "Não foi possível carregar as categorias.",
    { signal },
  );
  if (!isRecord(body) || !Array.isArray(body.data) || !body.data.every(isCategory)) {
    throw new Error("A resposta de categorias é inválida.");
  }
  return body.data;
}

export async function searchProducts(
  search: ProductSearch,
  signal: AbortSignal,
): Promise<ProductPage> {
  const params = buildProductSearchParams(search);

  const body = await requestJson(
    `/api/v1/products?${params.toString()}`,
    "Não foi possível pesquisar o catálogo.",
    { headers: { Accept: "application/json" }, signal },
  );
  if (!isProductPage(body)) {
    throw new Error("A resposta da API não corresponde ao contrato de pesquisa.");
  }
  return body;
}

export function buildProductSearchParams(search: ProductSearch): URLSearchParams {
  const params = new URLSearchParams({
    q: search.q,
    page: String(search.page),
    page_size: "24",
    sort: search.sort,
  });
  if (search.categoryId) params.set("category_id", search.categoryId);
  if (search.brand.trim()) params.set("brand", search.brand.trim());
  if (search.minPrice) params.set("min_price", search.minPrice);
  if (search.maxPrice) params.set("max_price", search.maxPrice);
  return params;
}

export async function getProduct(
  id: string,
  signal?: AbortSignal,
): Promise<ProductDetail> {
  const body = await requestJson(
    `/api/v1/products/${encodeURIComponent(id)}`,
    "Não foi possível abrir o produto.",
    { headers: { Accept: "application/json" }, signal },
  );
  if (!isProductDetail(body)) {
    throw new Error("A resposta da API não corresponde ao contrato de produto.");
  }
  return body;
}
