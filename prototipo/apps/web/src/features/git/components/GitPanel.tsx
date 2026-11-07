import { useEffect, useState } from "react";
import { createLocalCommit, getGitStatus, type GitStatus } from "../api";

export default function GitPanel({ onClose }: { onClose: () => void }) {
  const [status, setStatus] = useState<GitStatus | null>(null);
  const [selected, setSelected] = useState<string[]>([]);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [loading, setLoading] = useState(false);
  const files = status?.files ?? [];

  async function refresh() {
    setLoading(true);
    setError("");
    try {
      const current = await getGitStatus();
      setStatus(current);
      setSelected([]);
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : "Falha ao consultar o Git.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void refresh();
  }, []);

  async function createCommit() {
    if (!selected.length || !message.trim()) {
      setError("Selecione arquivos e informe a mensagem do commit.");
      return;
    }
    if (
      !window.confirm(
        `Criar commit local para ${selected.length} arquivo(s)? Nada será enviado ao GitHub.`,
      )
    ) {
      return;
    }

    setLoading(true);
    setError("");
    setNotice("");
    try {
      const commit = await createLocalCommit(selected, message.trim());
      setNotice(`Commit local criado: ${commit}`);
      setMessage("");
      await refresh();
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : "O commit local não foi criado.");
    } finally {
      setLoading(false);
    }
  }

  function toggle(path: string, checked: boolean) {
    setSelected((current) => {
      if (checked && current.length >= 200) {
        setError("O limite por commit é de 200 arquivos.");
        return current;
      }
      setError("");
      return checked ? [...current, path] : current.filter((value) => value !== path);
    });
  }

  return (
    <div className="git-overlay" onMouseDown={(event) => {
      if (event.target === event.currentTarget) onClose();
    }}>
      <section
        aria-labelledby="git-title"
        aria-modal="true"
        className="git-panel"
        role="dialog"
      >
        <button className="git-close" type="button" onClick={onClose} aria-label="Fechar">
          ×
        </button>
        <p className="eyebrow">CONTROLE LOCAL</p>
        <h2 id="git-title">Commit do projeto</h2>
        <p className="git-description">
          Escolha os arquivos e confirme para criar um commit no repositório
          local. O push para o GitHub é sempre manual.
        </p>

        {error && <p className="git-alert git-alert-error" role="alert">{error}</p>}
        {notice && <p className="git-alert git-alert-success" role="status">{notice}</p>}

        <div className="git-files-heading">
          <strong>{loading ? "Atualizando status..." : `${files.length} arquivo(s) alterado(s)`}</strong>
          <div className="git-file-actions">
            {files.length > 0 && (
              <button
                disabled={loading}
                type="button"
                onClick={() =>
                  setSelected(
                    selected.length === Math.min(files.length, 200)
                      ? []
                      : files.slice(0, 200).map((file) => file.path),
                  )
                }
              >
                {selected.length === Math.min(files.length, 200) ? "Desmarcar seleção" : "Selecionar até 200"}
              </button>
            )}
            <button disabled={loading} type="button" onClick={() => void refresh()}>
              Atualizar
            </button>
          </div>
        </div>
        {files.length > 0 ? (
          <div className="git-files">
            {files.map((file) => (
              <label className="git-file-row" key={file.path}>
                <input
                  checked={selected.includes(file.path)}
                  disabled={loading}
                  onChange={(event) => toggle(file.path, event.target.checked)}
                  type="checkbox"
                />
                <span className="git-file-status">{file.status}</span>
                <code title={file.path}>{file.path}</code>
              </label>
            ))}
          </div>
        ) : (
          <div className="git-no-files">
            {error ? "O status não foi carregado." : "Nenhuma alteração neste projeto."}
          </div>
        )}

        <label className="git-message-label" htmlFor="git-message">Mensagem do commit</label>
        <input
          id="git-message"
          className="git-message-input"
          maxLength={120}
          onChange={(event) => setMessage(event.target.value)}
          placeholder="Ex.: atualiza busca do catálogo"
          value={message}
        />
        <div className="git-panel-actions">
          <button className="git-secondary" type="button" onClick={onClose}>Fechar</button>
          <button
            className="git-primary"
            disabled={loading || selected.length === 0 || !message.trim()}
            onClick={() => void createCommit()}
            type="button"
          >
            Criar commit <span aria-hidden="true">→</span>
          </button>
        </div>
        <p className="git-footnote">
          O commit usa a configuração Git existente, não armazena token do GitHub e
          inclui somente os arquivos selecionados.
        </p>
      </section>
    </div>
  );
}
