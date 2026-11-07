import { isRecord } from "../../api/http";

export type HealthResponse = { status: string; service: string; version: string };
export type Category = { id: string; name: string; parent_id: string | null };
export type Sort = "relevance" | "name_asc" | "price_asc" | "price_desc";
export type Product = {
  id: string;
  name: string;
  brand: string | null;
  category: { id: string; name: string } | null;
  unit: string | null;
  offers_count: number;
};
export type ProductPage = {
  data: Product[];
  pagination: { page: number; page_size: number; total_items: number; total_pages: number };
};
export type Offer = {
  id: string;
  store: string;
  price: string | null;
  currency: string;
  price_type: string;
  fulfillment: string;
  availability: string;
  conditions: string | null;
  offer_url: string;
  observed_at: string;
};
export type ProductDetail = Product & {
  variant: string | null;
  model_code: string | null;
  offers: Offer[];
};
export type ProductSearch = {
  q: string;
  page: number;
  sort: Sort;
  categoryId: string;
  brand: string;
  minPrice: string;
  maxPrice: string;
};

export function isHealthResponse(value: unknown): value is HealthResponse {
  return (
    isRecord(value) &&
    typeof value.status === "string" &&
    typeof value.service === "string" &&
    typeof value.version === "string"
  );
}

export function isCategory(value: unknown): value is Category {
  return (
    isRecord(value) &&
    typeof value.id === "string" &&
    typeof value.name === "string" &&
    (typeof value.parent_id === "string" || value.parent_id === null)
  );
}

export function isProduct(value: unknown): value is Product {
  return (
    isRecord(value) &&
    typeof value.id === "string" &&
    typeof value.name === "string" &&
    (typeof value.brand === "string" || value.brand === null) &&
    (value.category === null ||
      (isRecord(value.category) &&
        typeof value.category.id === "string" &&
        typeof value.category.name === "string")) &&
    (typeof value.unit === "string" || value.unit === null) &&
    typeof value.offers_count === "number"
  );
}

export function isProductPage(value: unknown): value is ProductPage {
  if (!isRecord(value) || !Array.isArray(value.data) || !isRecord(value.pagination)) {
    return false;
  }
  const pagination = value.pagination;
  return (
    value.data.every(isProduct) &&
    Number.isInteger(pagination.page) &&
    Number.isInteger(pagination.page_size) &&
    Number.isInteger(pagination.total_items) &&
    Number.isInteger(pagination.total_pages)
  );
}

function isOffer(value: unknown): value is Offer {
  return (
    isRecord(value) &&
    typeof value.id === "string" &&
    typeof value.store === "string" &&
    (typeof value.price === "string" || value.price === null) &&
    typeof value.currency === "string" &&
    typeof value.price_type === "string" &&
    typeof value.fulfillment === "string" &&
    typeof value.availability === "string" &&
    (typeof value.conditions === "string" || value.conditions === null) &&
    typeof value.offer_url === "string" &&
    typeof value.observed_at === "string"
  );
}

export function isProductDetail(value: unknown): value is ProductDetail {
  if (
    !isRecord(value) ||
    !("variant" in value) ||
    !("model_code" in value) ||
    !("offers" in value) ||
    (typeof value.variant !== "string" && value.variant !== null) ||
    (typeof value.model_code !== "string" && value.model_code !== null) ||
    !Array.isArray(value.offers) ||
    !value.offers.every(isOffer)
  ) {
    return false;
  }
  return isProduct(value);
}
