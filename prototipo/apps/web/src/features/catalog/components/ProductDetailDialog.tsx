import type { ProductDetail } from "../types";

export default function ProductDetailDialog({
  product,
  onClose,
}: {
  product: ProductDetail;
  onClose: () => void;
}) {
  return (
    <div
      className="product-overlay"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <section className="product-dialog" role="dialog" aria-modal="true" aria-labelledby="product-title">
        <button className="git-close" type="button" onClick={onClose} aria-label="Fechar">×</button>
        <p className="eyebrow">PRODUTO RASTREÁVEL</p>
        <h2 id="product-title">{product.name}</h2>
        <p className="product-detail-meta">
          {product.brand ?? "Marca não informada"} · {product.unit ?? "Unidade não informada"}
          {product.variant ? ` · ${product.variant}` : ""}
          {product.model_code ? ` · ${product.model_code}` : ""}
        </p>
        <h3>Ofertas publicáveis</h3>
        {product.offers.length === 0 ? (
          <p className="search-state">Nenhuma oferta atual confirmada para este produto.</p>
        ) : (
          <ul className="offer-list">
            {product.offers.map((offer) => (
              <li key={offer.id}>
                <strong>{offer.store}</strong>
                <span>{offer.price ? `R$ ${offer.price}` : "Preço não informado"}</span>
                <small>{offer.price_type} · {offer.fulfillment} · Observada em {formatDate(offer.observed_at)}</small>
                {offer.conditions && <small>{offer.conditions}</small>}
                <a href={offer.offer_url} target="_blank" rel="noreferrer">Consultar fonte original ↗</a>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}

function formatDate(value: string): string {
  const parsed = new Date(value);
  return Number.isNaN(parsed.getTime()) ? value : parsed.toLocaleDateString("pt-BR");
}
