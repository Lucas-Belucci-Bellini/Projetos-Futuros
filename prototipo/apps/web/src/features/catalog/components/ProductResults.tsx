import type { Product, Sort } from "../types";

type Props = {
  query: string;
  results: Product[] | null;
  resultCount: number;
  searching: boolean;
  searchError: string;
  sort: Sort;
  page: number;
  totalPages: number;
  onSortChange: (sort: Sort) => void;
  onPageChange: (page: number) => void;
  onSelectProduct: (product: Product) => void;
};

export default function ProductResults({
  query,
  results,
  resultCount,
  searching,
  searchError,
  sort,
  page,
  totalPages,
  onSortChange,
  onPageChange,
  onSelectProduct,
}: Props) {
  if (!searching && !searchError && results === null) return null;

  return (
    <section className="search-results mx-auto max-w-7xl px-5 py-10" aria-live="polite">
      <div className="results-heading">
        <div>
          <p className="eyebrow">BUSCA NO CATÁLOGO</p>
          <h2>Resultados para “{query.trim()}”</h2>
        </div>
        {!searching && !searchError && <span>{resultCount} resultado(s)</span>}
        {!searching && !searchError && results && results.length > 0 && (
          <label className="results-sort">
            Ordenar
            <select value={sort} onChange={(event) => onSortChange(event.target.value as Sort)}>
              <option value="relevance">Relevância</option>
              <option value="name_asc">Nome A–Z</option>
              <option value="price_asc">Menor preço</option>
              <option value="price_desc">Maior preço</option>
            </select>
          </label>
        )}
      </div>
      {searching && <p className="search-state">Pesquisando materiais...</p>}
      {searchError && <p className="search-state search-error" role="alert">{searchError}</p>}
      {!searching && !searchError && results?.length === 0 && (
        <div className="search-empty">
          <strong>Nenhum produto publicado foi encontrado.</strong>
          <p>Os resultados só aparecem quando a fonte está aprovada e os dados foram revisados.</p>
        </div>
      )}
      {!searching && !searchError && results && results.length > 0 && (
        <>
          <div className="search-product-grid">
            {results.map((product) => (
              <button
                className="search-product-card"
                key={product.id}
                onClick={() => onSelectProduct(product)}
                type="button"
              >
                <span>{product.category?.name ?? "Material"}</span>
                <strong>{product.name}</strong>
                <small>{product.brand ?? "Marca não informada"} · {product.unit ?? "Unidade não informada"}</small>
                <small>{product.offers_count} oferta(s) verificadas</small>
              </button>
            ))}
          </div>
          {totalPages > 1 && (
            <nav className="pagination" aria-label="Paginação dos resultados">
              <button disabled={page <= 1 || searching} onClick={() => onPageChange(page - 1)} type="button">← Anterior</button>
              <span>Página {page} de {totalPages}</span>
              <button disabled={page >= totalPages || searching} onClick={() => onPageChange(page + 1)} type="button">Próxima →</button>
            </nav>
          )}
        </>
      )}
    </section>
  );
}
