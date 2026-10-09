import { useEffect, useState, type FormEvent } from "react";

type HealthResponse = { status: string; service: string; version: string };

export default function App() {
  const [query, setQuery] = useState("");
  const [apiState, setApiState] = useState<"checking" | "online" | "offline">("checking");
  const [health, setHealth] = useState<HealthResponse | null>(null);

  useEffect(() => {
    const controller = new AbortController();
    fetch("/api/v1/health", { signal: controller.signal })
      .then(async (response) => {
        if (!response.ok) throw new Error("API indisponível");
        return (await response.json()) as HealthResponse;
      })
      .then((data) => { setHealth(data); setApiState("online"); })
      .catch((error: unknown) => {
        if (error instanceof Error && error.name === "AbortError") return;
        setApiState("offline");
      });
    return () => controller.abort();
  }, []);

  function submitSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const cleaned = query.trim();
    if (!cleaned) return;
    const params = new URLSearchParams({ q: cleaned });
    window.location.hash = `/buscar?${params.toString()}`;
  }

  return (
    <main className="min-h-screen">
      <header className="border-b border-stone-200 bg-white">
        <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-5 py-4">
          <a className="brand" href="#" aria-label="Catálogo de Materiais — início">
            <span className="brand-mark" aria-hidden="true">CM</span>
            <span><strong>Materiais</strong><small>Londrina e região</small></span>
          </a>
          <span className="api-status" aria-live="polite"><span className={`status-dot status-${apiState}`} />{apiState === "checking" ? "Conectando à API" : apiState === "online" ? "API conectada" : "API offline"}</span>
        </div>
      </header>
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
      <section className="mx-auto max-w-7xl px-5 py-14">
        <div className="section-heading"><div><p className="eyebrow">EXPLORAÇÃO</p><h2>Por onde começar?</h2></div><p>As categorias serão preenchidas com produtos verificados à medida que as fontes forem aprovadas.</p></div>
        <div className="category-grid">
          {[["01", "Materiais básicos", "Cimento, argamassa e cal"], ["02", "Elétrica e hidráulica", "Tubos, conexões e cabos"], ["03", "Pisos e acabamentos", "Revestimentos e acessórios"], ["04", "Ferramentas", "Itens para instalação e obra"]].map(([number, title, description]) => <article className="category-card" key={number}><span>{number}</span><h3>{title}</h3><p>{description}</p><small>Categoria planejada</small></article>)}
        </div>
      </section>
      <footer className="site-footer"><div className="mx-auto flex max-w-7xl flex-col gap-2 px-5 py-7 sm:flex-row sm:items-center sm:justify-between"><span>Catálogo de Materiais — Londrina/PR</span><span>Projeto independente · Dados sujeitos a verificação</span></div></footer>
    </main>
  );
}
