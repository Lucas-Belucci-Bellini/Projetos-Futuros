import { useEffect, useState, type ChangeEvent, type FormEvent } from "react";
import { isRecord, requestJson } from "../../api/http";

type DocumentSummary = {
  id: number;
  title: string;
  excerpt: string;
  tags: string;
  updated_at: string;
};

type Document = {
  id: number;
  title: string;
  content: string;
  tags: string;
  created_at: string;
  updated_at: string;
};

type HermesStatus = {
  installed: boolean;
  version: string | null;
};

type ResearchResponse = {
  answer: string;
};

const acceptedExtensions = new Set([".txt", ".md", ".markdown", ".csv", ".json", ".html"]);
const maxFileSize = 48 * 1024;

function isDocumentSummary(value: unknown): value is DocumentSummary {
  return (
    isRecord(value) &&
    typeof value.id === "number" &&
    typeof value.title === "string" &&
    typeof value.excerpt === "string" &&
    typeof value.tags === "string" &&
    typeof value.updated_at === "string"
  );
}

function isDocument(value: unknown): value is Document {
  return (
    isRecord(value) &&
    typeof value.id === "number" &&
    typeof value.title === "string" &&
    typeof value.content === "string" &&
    typeof value.tags === "string" &&
    typeof value.created_at === "string" &&
    typeof value.updated_at === "string"
  );
}

function isHermesStatus(value: unknown): value is HermesStatus {
  return (
    isRecord(value) &&
    typeof value.installed === "boolean" &&
    (typeof value.version === "string" || value.version === null)
  );
}

function isResearchResponse(value: unknown): value is ResearchResponse {
  return isRecord(value) && typeof value.answer === "string";
}

export default function LibraryWorkspace() {
  const [documents, setDocuments] = useState<DocumentSummary[]>([]);
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState<number | null>(null);
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [tags, setTags] = useState("");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [hermesStatus, setHermesStatus] = useState<HermesStatus | null>(null);
  const [question, setQuestion] = useState("");
  const [researchAnswer, setResearchAnswer] = useState("");
  const [researching, setResearching] = useState(false);

  async function loadDocuments(search = query) {
    setLoading(true);
    setError("");
    try {
      const params = new URLSearchParams();
      if (search.trim()) params.set("q", search.trim());
      const body = await requestJson(
        `/api/v1/library/documents?${params.toString()}`,
        "Não foi possível carregar sua biblioteca.",
      );
      if (!Array.isArray(body) || !body.every(isDocumentSummary)) {
        throw new Error("A resposta da biblioteca é inválida.");
      }
      setDocuments(body);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Não foi possível carregar sua biblioteca.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadDocuments("");
    requestJson("/api/v1/hermes/status", "Não foi possível verificar o Hermes.")
      .then((body) => {
        if (!isHermesStatus(body)) throw new Error("A resposta de status do Hermes é inválida.");
        setHermesStatus(body);
      })
      .catch((caught: unknown) => {
        setHermesStatus({ installed: false, version: null });
        setError((current) => current || (caught instanceof Error ? caught.message : "Não foi possível verificar o Hermes."));
      });
  }, []);

  function startNewDocument() {
    setSelectedId(null);
    setTitle("");
    setContent("");
    setTags("");
    setNotice("");
  }

  async function selectDocument(id: number) {
    setError("");
    setNotice("");
    try {
      const body = await requestJson(
        `/api/v1/library/documents/${id}`,
        "Não foi possível abrir o registro.",
      );
      if (!isDocument(body)) throw new Error("A resposta do registro é inválida.");
      setSelectedId(body.id);
      setTitle(body.title);
      setContent(body.content);
      setTags(body.tags);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Não foi possível abrir o registro.");
    }
  }

  async function saveDocument(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSaving(true);
    setError("");
    setNotice("");
    try {
      const body = await requestJson(
        selectedId === null
          ? "/api/v1/library/documents"
          : `/api/v1/library/documents/${selectedId}`,
        "Não foi possível salvar o registro.",
        {
          method: selectedId === null ? "POST" : "PUT",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ title, content, tags }),
        },
      );
      if (!isDocument(body)) throw new Error("A resposta do registro salvo é inválida.");
      setSelectedId(body.id);
      setTitle(body.title);
      setContent(body.content);
      setTags(body.tags);
      setNotice("Registro salvo somente neste computador.");
      await loadDocuments(query);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Não foi possível salvar o registro.");
    } finally {
      setSaving(false);
    }
  }

  async function deleteDocument() {
    if (selectedId === null || !window.confirm("Excluir este registro da biblioteca local?")) return;
    setError("");
    setNotice("");
    try {
      await requestJson(
        `/api/v1/library/documents/${selectedId}`,
        "Não foi possível excluir o registro.",
        { method: "DELETE" },
      );
      startNewDocument();
      setNotice("Registro excluído da biblioteca local.");
      await loadDocuments(query);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Não foi possível excluir o registro.");
    }
  }

  async function readSelectedFile(event: ChangeEvent<HTMLInputElement>) {
    const file = event.currentTarget.files?.[0];
    event.currentTarget.value = "";
    if (!file) return;
    const extension = file.name.includes(".")
      ? file.name.slice(file.name.lastIndexOf(".")).toLowerCase()
      : "";
    if (!acceptedExtensions.has(extension)) {
      setError("Formato não suportado. Escolha um arquivo TXT, Markdown, CSV, JSON ou HTML.");
      return;
    }
    if (file.size > maxFileSize) {
      setError("O arquivo selecionado deve ter no máximo 48 KB neste protótipo.");
      return;
    }
    try {
      const text = await file.text();
      if (!text.trim()) throw new Error("O arquivo selecionado está vazio.");
      if (text.length > 12000) {
        throw new Error("O texto do arquivo ultrapassa o limite de 12.000 caracteres.");
      }
      setSelectedId(null);
      setTitle(file.name.slice(0, 160));
      setContent(text);
      setTags("");
      setError("");
      setNotice("Arquivo carregado no formulário. Revise e salve para adicionar à biblioteca local.");
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Não foi possível ler o arquivo selecionado.");
    }
  }

  async function searchWithHermes(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setResearching(true);
    setError("");
    setResearchAnswer("");
    try {
      const body = await requestJson(
        "/api/v1/hermes/research",
        "Hermes não concluiu a pesquisa.",
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ question }),
        },
      );
      if (!isResearchResponse(body)) throw new Error("A resposta de pesquisa do Hermes é inválida.");
      setResearchAnswer(body.answer);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Hermes não concluiu a pesquisa.");
    } finally {
      setResearching(false);
    }
  }

  return (
    <section className="library-workspace mx-auto max-w-7xl px-5 py-12">
      <div className="library-intro">
        <p className="eyebrow">SEUS ARQUIVOS, NO SEU COMPUTADOR</p>
        <h1>Uma biblioteca que você consegue encontrar.</h1>
        <p>
          Guarde notas e importe arquivos de texto escolhidos por você. A busca e os registros
          ficam no SQLite local; nada é sincronizado ou enviado automaticamente.
        </p>
      </div>

      <div className="library-layout">
        <aside className="library-list-panel" aria-label="Registros da biblioteca">
          <div className="library-list-heading">
            <h2>Minha biblioteca</h2>
            <button type="button" className="library-secondary-button" onClick={startNewDocument}>
              Novo
            </button>
          </div>
          <form
            className="library-search"
            onSubmit={(event) => {
              event.preventDefault();
              void loadDocuments(query);
            }}
          >
            <label className="sr-only" htmlFor="library-search">Pesquisar na biblioteca</label>
            <input
              id="library-search"
              maxLength={160}
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Buscar notas, texto e etiquetas"
            />
            <button type="submit">Buscar</button>
          </form>
          {loading && <p className="library-muted">Carregando registros…</p>}
          {!loading && documents.length === 0 && (
            <p className="library-muted">Nenhum registro ainda. Crie uma nota ou importe um arquivo.</p>
          )}
          <div className="library-document-list">
            {documents.map((document) => (
              <button
                type="button"
                key={document.id}
                className={`library-document-item${selectedId === document.id ? " is-selected" : ""}`}
                onClick={() => void selectDocument(document.id)}
              >
                <strong>{document.title}</strong>
                <span>{document.excerpt}</span>
                {document.tags && <small>{document.tags}</small>}
              </button>
            ))}
          </div>
        </aside>

        <div className="library-main">
          <form className="library-editor" onSubmit={(event) => void saveDocument(event)}>
            <div className="library-editor-heading">
              <div>
                <p className="eyebrow">{selectedId === null ? "NOVO REGISTRO" : "EDITANDO REGISTRO"}</p>
                <h2>{selectedId === null ? "Anote ou guarde um arquivo" : "Organize este registro"}</h2>
              </div>
              {selectedId !== null && (
                <button type="button" className="library-danger-button" onClick={() => void deleteDocument()}>
                  Excluir
                </button>
              )}
            </div>
            <label>
              Título
              <input
                required
                maxLength={160}
                value={title}
                onChange={(event) => setTitle(event.target.value)}
                placeholder="Ex.: orçamento da reforma"
              />
            </label>
            <label>
              Conteúdo
              <textarea
                required
                maxLength={12000}
                rows={12}
                value={content}
                onChange={(event) => setContent(event.target.value)}
                placeholder="Escreva uma nota ou revise o texto do arquivo importado."
              />
              <small>{content.length.toLocaleString("pt-BR")} / 12.000 caracteres</small>
            </label>
            <label>
              Etiquetas
              <input
                maxLength={300}
                value={tags}
                onChange={(event) => setTags(event.target.value)}
                placeholder="obra, orçamento, casa"
              />
            </label>
            <div className="library-editor-actions">
              <label className="library-file-button">
                Importar arquivo de texto
                <input
                  type="file"
                  accept=".txt,.md,.markdown,.csv,.json,.html,text/plain,text/markdown,text/csv,application/json,text/html"
                  onChange={(event) => void readSelectedFile(event)}
                />
              </label>
              <button type="submit" className="library-primary-button" disabled={saving}>
                {saving ? "Salvando…" : "Salvar neste computador"}
              </button>
            </div>
            <p className="library-muted">
              Arquivos de texto até 48 KB e 12.000 caracteres. O original não é movido nem alterado.
            </p>
            {error && <p className="search-error" role="alert">{error}</p>}
            {notice && <p className="library-notice" role="status">{notice}</p>}
          </form>

          <section className="hermes-card" aria-labelledby="hermes-title">
            <div className="hermes-card-heading">
              <div>
                <p className="eyebrow">PESQUISA EXTERNA OPCIONAL</p>
                <h2 id="hermes-title">Pesquisar com Hermes</h2>
              </div>
              <span className={`hermes-status${hermesStatus?.installed ? " is-installed" : ""}`}>
                {hermesStatus === null ? "Verificando…" : hermesStatus.installed ? "Instalado" : "Não instalado"}
              </span>
            </div>
            {hermesStatus?.version && <p className="library-muted">{hermesStatus.version}</p>}
            <p className="library-muted">
              Sua pergunta será enviada ao Hermes e à web. O conteúdo desta biblioteca não é enviado.
              A pesquisa usa apenas as ferramentas web do Hermes, sem terminal.
            </p>
            {!hermesStatus?.installed && (
              <p className="library-muted">
                No Windows, instale pelo{" "}
                <a href="https://hermes-agent.nousresearch.com/docs/getting-started/installation" target="_blank" rel="noreferrer">
                  instalador oficial do Hermes
                </a>
                {" "}e conclua a configuração com <code>hermes setup</code>.
              </p>
            )}
            <form className="hermes-research-form" onSubmit={(event) => void searchWithHermes(event)}>
              <label className="sr-only" htmlFor="hermes-question">O que você quer pesquisar na web?</label>
              <input
                id="hermes-question"
                required
                maxLength={500}
                value={question}
                onChange={(event) => setQuestion(event.target.value)}
                placeholder="Ex.: como organizar notas de manutenção?"
              />
              <button type="submit" className="library-primary-button" disabled={!hermesStatus?.installed || researching}>
                {researching ? "Pesquisando…" : "Pesquisar na web"}
              </button>
            </form>
            {researchAnswer && <pre className="hermes-answer">{researchAnswer}</pre>}
          </section>
        </div>
      </div>
    </section>
  );
}
