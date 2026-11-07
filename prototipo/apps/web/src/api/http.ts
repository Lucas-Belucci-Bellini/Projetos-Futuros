export async function requestJson(
  url: string,
  fallbackMessage: string,
  init?: RequestInit,
): Promise<unknown> {
  const headers = new Headers(init?.headers);
  headers.set("Accept", "application/json");
  const response = await fetch(url, { ...init, headers });
  if (response.status === 204) return undefined;
  const body: unknown = await response.json();
  if (!response.ok) {
    throw new Error(readApiError(body, fallbackMessage));
  }
  return body;
}

export function readApiError(body: unknown, fallback: string): string {
  if (
    isRecord(body) &&
    isRecord(body.error) &&
    typeof body.error.message === "string"
  ) {
    return body.error.message;
  }
  return fallback;
}

export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}
