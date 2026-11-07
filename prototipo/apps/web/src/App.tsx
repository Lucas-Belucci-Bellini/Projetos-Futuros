import { useEffect, useRef, useState, type FormEvent } from "react";
import GitPanel from "./features/git/components/GitPanel";
import { getCategories, getHealth, getProduct, searchProducts, buildProductSearchParams } from "./features/catalog/api";
import type { Category, HealthResponse, Product, ProductDetail, Sort } from "./features/catalog/types";
import ProductDetailDialog from "./features/catalog/components/ProductDetailDialog";
import ProductResults from "./features/catalog/components/ProductResults";
import LibraryWorkspace from "./features/library/LibraryWorkspace";

export default function App() {
  const [query, setQuery] = useState("");
  const [categories, setCategories] = useState<Category[]>([]);
  const [categoryError, setCategoryError] = useState("");
  const [categoryId, setCategoryId] = useState("");
  const [brand, setBrand] = useState("");
  const [minPrice, setMinPrice] = useState("");
  const [maxPrice, setMaxPrice] = useState("");
  const [sort, setSort] = useState<Sort>("relevance");
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(0);
  const [results, setResults] = useState<Product[] | null>(null);
  const [resultCount, setResultCount] = useState(0);
  const [searching, setSearching] = useState(false);
  const [searchError, setSearchError] = useState("");
  const [gitOpen, setGitOpen] = useState(false);
  const [libraryOpen, setLibraryOpen] = useState(false);
  const [selectedProduct, setSelectedProduct] = useState<ProductDetail | null>(null);
  const [detailError, setDetailError] = useState("");
  const [apiState, setApiState] = useState<"checking" | "online" | "offline">("checking");
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const searchController = useRef<AbortController | null>(null);

  useEffect(() => {
    const controller = new AbortController();
    getHealth(controller.signal)
      .then((data) => { setHealth(data); setApiState("online"); })
      .catch((error: unknown) => {
        if (error instanceof Error && error.name === "AbortError") return;
        setApiState("offline");
      });
    return () => controller.abort();
  }, []);

  useEffect(() => {
    const controller = new AbortController();
    getCategories(controller.signal)
      .then(setCategories)
      .catch((error: unknown) => {
        if (error instanceof Error && error.name === "AbortError") return;
        setCategoryError(error instanceof Error ? error.message : "Não foi possível carregar as categorias.");
        setCategories([]);
      });
    return () => controller.abort();
  }, []);

  useEffect(() => () => searchController.current?.abort(), []);

  async function searchCatalog(requestedPage: number, requestedSort = sort) {
    const cleaned = query.trim();
    if (!cleaned) {
      setSearchError("Digite o nome, a marca ou o código do material.");
      return;
    }
    const search = {
      q: cleaned,
      page: requestedPage,
      sort: requestedSort,
      categoryId,
      brand,
      minPrice,
      maxPrice,
    };
    const params = buildProductSearchParams(search);
    window.location.hash = `/buscar?${params.toString()}`;
    searchController.current?.abort();
    const controller = new AbortController();
    searchController.current = controller;
    setSearchError("");
    setSearching(true);
    try {
      const page = await searchProducts(search, controller.signal);
      setResults(page.data);
      setResultCount(page.pagination.total_items);
      setPage(page.pagination.page);
      setTotalPages(page.pagination.total_pages);
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") return;
      setResults(null);
      setSearchError(error instanceof Error ? error.message : "Não foi possível pesquisar o catálogo.");
    } finally {
      if (searchController.current === controller) setSearching(false);
    }
  }

  function submitSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void searchCatalog(1);
  }

  async function openProduct(product: Product) {
    setSelectedProduct(null);
    setDetailError("");
    try {
      setSelectedProduct(await getProduct(product.id));
    } catch (error) {
      setDetailError(error instanceof Error ? error.message : "Não foi possível abrir o produto.");
    }
  }

  return (
    <main className="min-h-screen">
      <header className="border-b border-stone-200 bg-white">
        <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-5 py-4">
          <a className="brand" href="#" aria-label="Catálogo de Materiais — início">
            <span className="brand-mark" aria-hidden="true">CM</span>
            <span><strong>Materiais</strong><small>Londrina e região</small></span>
          </a>
          <div className="header-actions">
            <span className="api-status" aria-live="polite"><span className={`status-dot status-${apiState}`} />{apiState === "checking" ? "Conectando à API" : apiState === "online" ? "API conectada" : "API offline"}</span>
            <button className="library-open-button" onClick={() => setLibraryOpen((open) => !open)} type="button">
              {libraryOpen ? "Catálogo" : "Minha biblioteca"}
            </button>
            <button className="git-open-button" onClick={() => setGitOpen(true)} type="button">Git do projeto</button>
          </div>
        </div>
      </header>
      {libraryOpen ? <LibraryWorkspace /> : <>
      <section className="hero">
        <div className="mx-auto grid max-w-7xl items-center gap-10 px-5 py-16 md:grid-cols-[1.2fr_0.8fr] md:py-24">
          <div>
            <p className="eyebrow">CONSTRUÇÃO • REFORMA • ACABAMENTO</p>
            <h1>Encontre o material certo para sua obra.</h1>
            <p className="hero-copy">Pesquise materiais e compare ofertas de lojas que atendem Londrina. Preços e disponibilidade serão exibidos com fonte e data de verificação.</p>
            <form className="search-form" onSubmit={submitSearch}>
              <label className="sr-only" htmlFor="catalog-search">Qual material você procura?</label>
              <input id="catalog-search" value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Ex.: cimento, torneira, piso..." />
              <button type="submit">Pesquisar</button>
            </form>
            <details className="search-filters">
              <summary>Filtrar por categoria, marca e preço</summary>
              <div className="filter-fields">
                <label>
                  Categoria
                  <select value={categoryId} onChange={(event) => setCategoryId(event.target.value)}>
                    <option value="">Todas</option>
                    {categories.map((category) => (
                      <option key={category.id} value={category.id}>{category.name}</option>
                    ))}
                  </select>
                </label>
                <label>
                  Marca
                  <input value={brand} onChange={(event) => setBrand(event.target.value)} maxLength={120} placeholder="Qualquer marca" />
                </label>
                <label>
                  Preço mínimo (R$)
                  <input type="number" min="0" step="0.01" value={minPrice} onChange={(event) => setMinPrice(event.target.value)} />
                </label>
                <label>
                  Preço máximo (R$)
                  <input type="number" min="0" step="0.01" value={maxPrice} onChange={(event) => setMaxPrice(event.target.value)} />
                </label>
              </div>
            </details>
            {categoryError && <p className="search-error" role="alert">{categoryError}</p>}
            <p className="privacy-note">Catálogo independente. A disponibilidade precisa ser confirmada na loja.</p>
          </div>
          <div className="hero-panel">
            <div className="panel-icon" aria-hidden="true">⌂</div>
            <p className="panel-kicker">UM CATÁLOGO EM CONSTRUÇÃO</p>
            <h2>Mais clareza na hora de comprar.</h2>
            <ul><li>Produtos organizados por categoria</li><li>Comparação de ofertas equivalentes</li><li>Origem e data de observação visíveis</li></ul>
            <div className="panel-footer">{apiState === "online" && health ? `Serviço ${health.service} · versão ${health.version}` : "Integração de catálogo em desenvolvimento"}</div>
          </div>
        </div>
      </section>
      <ProductResults
        query={query}
        results={results}
        resultCount={resultCount}
        searching={searching}
        searchError={searchError}
        sort={sort}
        page={page}
        totalPages={totalPages}
        onSortChange={(nextSort) => {
          setSort(nextSort);
          void searchCatalog(1, nextSort);
        }}
        onPageChange={(nextPage) => void searchCatalog(nextPage)}
        onSelectProduct={(product) => void openProduct(product)}
      />
      <section className="mx-auto max-w-7xl px-5 py-14">
        <div className="section-heading"><div><p className="eyebrow">EXPLORAÇÃO</p><h2>Por onde começar?</h2></div><p>As categorias serão preenchidas com produtos verificados à medida que as fontes forem aprovadas.</p></div>
        <div className="category-grid">
          {[["01", "Materiais básicos", "Cimento, argamassa e cal"], ["02", "Elétrica e hidráulica", "Tubos, conexões e cabos"], ["03", "Pisos e acabamentos", "Revestimentos e acessórios"], ["04", "Ferramentas", "Itens para instalação e obra"]].map(([number, title, description]) => <article className="category-card" key={number}><span>{number}</span><h3>{title}</h3><p>{description}</p><small>Categoria planejada</small></article>)}
        </div>
      </section>
      <footer className="site-footer"><div className="mx-auto flex max-w-7xl flex-col gap-2 px-5 py-7 sm:flex-row sm:items-center sm:justify-between"><span>Catálogo de Materiais — Londrina/PR</span><span>Projeto independente · Dados sujeitos a verificação</span></div></footer>
      {gitOpen && <GitPanel onClose={() => setGitOpen(false)} />}
      {selectedProduct && (
        <ProductDetailDialog
          product={selectedProduct}
          onClose={() => setSelectedProduct(null)}
        />
      )}
      {detailError && !selectedProduct && (
        <p className="detail-toast" role="alert">
          {detailError}
          <button type="button" onClick={() => setDetailError("")} aria-label="Fechar aviso">×</button>
        </p>
      )}
      </>}
    </main>
  );
}
