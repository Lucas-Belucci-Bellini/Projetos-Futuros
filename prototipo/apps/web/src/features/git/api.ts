import { requestJson, isRecord } from "../../api/http";

export type GitFile = { path: string; status: string };
export type GitStatus = { scope: string; files: GitFile[] };

function isGitFile(value: unknown): value is GitFile {
  return (
    isRecord(value) &&
    typeof value.path === "string" &&
    typeof value.status === "string"
  );
}

function isGitStatus(value: unknown): value is GitStatus {
  return (
    isRecord(value) &&
    typeof value.scope === "string" &&
    Array.isArray(value.files) &&
    value.files.every(isGitFile)
  );
}

export async function getGitStatus(): Promise<GitStatus> {
  const body = await requestJson(
    "/api/v1/git/status",
    "Não foi possível consultar o Git.",
  );
  if (!isGitStatus(body)) {
    throw new Error("A resposta de status Git é inválida.");
  }
  return body;
}

export async function createLocalCommit(
  files: string[],
  message: string,
): Promise<string> {
  const body = await requestJson(
    "/api/v1/git/commit",
    "O commit local não foi criado.",
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ files, message, confirmed: true }),
    },
  );
  if (!isRecord(body) || typeof body.commit !== "string") {
    throw new Error("A API não retornou o identificador do commit.");
  }
  return body.commit;
}
