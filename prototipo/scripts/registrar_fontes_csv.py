#!/usr/bin/env python3
"""Register discovered data sources without approving them automatically."""
from __future__ import annotations

import argparse
import csv
import json
import sqlite3
from pathlib import Path
from urllib.parse import urlparse

ALLOWED_TYPES = {"site", "api", "feed", "authorized_file", "manual"}
ALLOWED_STATUSES = {"discovered", "under_review", "restricted", "blocked", "inactive"}
REQUIRED = {"source_id", "organization_name", "official_url", "source_type", "access_method", "permission_status"}


def valid_url(value: str) -> bool:
    try:
        parsed = urlparse(value.strip())
        return parsed.scheme in {"http", "https"} and bool(parsed.netloc) and "." in parsed.netloc
    except ValueError:
        return False


def load_rows(path: Path) -> tuple[list[dict[str, str]], list[str]]:
    errors, rows = [], []
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        if not reader.fieldnames:
            return [], ["CSV sem cabeçalho"]
        missing = sorted(REQUIRED - {name.strip() for name in reader.fieldnames if name})
        if missing:
            return [], ["colunas obrigatórias ausentes: " + ", ".join(missing)]
        for line, raw in enumerate(reader, start=2):
            row = {(key or "").strip(): (value or "").strip() for key, value in raw.items() if key}
            row_errors = []
            for field in REQUIRED:
                if not row.get(field):
                    row_errors.append(f"linha {line}: campo obrigatório vazio: {field}")
            if row.get("official_url") and not valid_url(row["official_url"]):
                row_errors.append(f"linha {line}: official_url inválida")
            if row.get("product_listing_url") and not valid_url(row["product_listing_url"]):
                row_errors.append(f"linha {line}: product_listing_url inválida")
            if row.get("permission_evidence_url") and not valid_url(row["permission_evidence_url"]):
                row_errors.append(f"linha {line}: permission_evidence_url inválida")
            if row.get("source_type") not in ALLOWED_TYPES:
                row_errors.append(f"linha {line}: source_type não reconhecido")
            if row.get("permission_status") not in ALLOWED_STATUSES:
                row_errors.append(
                    f"linha {line}: status inválido; CSV não pode aprovar fontes automaticamente"
                )
            if row_errors:
                errors.extend(row_errors)
            else:
                rows.append(row)
    return rows, errors


def register_sources(csv_path: Path, db_path: Path, schema_path: Path) -> dict:
    rows, errors = load_rows(csv_path)
    report = {"file": str(csv_path), "rows_read": len(rows) + len(errors), "accepted_rows": len(rows),
              "inserted_rows": 0, "updated_rows": 0, "errors": errors, "status": "rejected"}
    if errors or not rows:
        if not errors:
            report["errors"].append("CSV não contém fontes.")
        return report
    try:
        connection = sqlite3.connect(db_path)
        try:
            connection.executescript(Path(schema_path).read_text(encoding="utf-8"))
            for row in rows:
                exists = connection.execute("SELECT 1 FROM sources WHERE id=?", (row["source_id"],)).fetchone()
                connection.execute(
                    """INSERT INTO sources
                       (id, organization_name, official_url, product_listing_url, source_type,
                        geographic_scope, access_method, permission_status, permission_evidence_url,
                        permitted_fields, media_permission, expected_refresh, last_checked_at, notes)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                       ON CONFLICT(id) DO UPDATE SET
                         organization_name=excluded.organization_name,
                         official_url=excluded.official_url,
                         product_listing_url=excluded.product_listing_url,
                         source_type=excluded.source_type,
                         geographic_scope=excluded.geographic_scope,
                         access_method=excluded.access_method,
                         permission_status=CASE
                           WHEN sources.official_url <> excluded.official_url
                             OR COALESCE(sources.product_listing_url, '') <> COALESCE(excluded.product_listing_url, '')
                             OR sources.source_type <> excluded.source_type
                             OR sources.access_method <> excluded.access_method
                             OR COALESCE(sources.permission_evidence_url, '') <> COALESCE(excluded.permission_evidence_url, '')
                             OR COALESCE(sources.permitted_fields, '') <> COALESCE(excluded.permitted_fields, '')
                             OR sources.media_permission <> excluded.media_permission
                           THEN 'under_review'
                           ELSE sources.permission_status
                         END,
                         permission_evidence_url=excluded.permission_evidence_url,
                         permitted_fields=excluded.permitted_fields,
                         media_permission=excluded.media_permission,
                         expected_refresh=excluded.expected_refresh,
                         last_checked_at=excluded.last_checked_at,
                         notes=excluded.notes,
                         updated_at=CURRENT_TIMESTAMP""",
                    (row["source_id"], row["organization_name"], row["official_url"],
                     row.get("product_listing_url") or None, row["source_type"],
                     row.get("geographic_scope") or "unknown", row["access_method"],
                     row["permission_status"], row.get("permission_evidence_url") or None,
                     row.get("permitted_fields") or None, row.get("media_permission") or "not_assessed",
                     row.get("expected_refresh") or None, row.get("last_checked_at") or None,
                     row.get("notes") or None),
                )
                report["updated_rows" if exists else "inserted_rows"] += 1
            connection.commit()
            report["status"] = "completed"
        except Exception:
            connection.rollback()
            raise
        finally:
            connection.close()
    except (OSError, sqlite3.Error) as exc:
        report["status"] = "failed"
        report["errors"].append(str(exc))
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description="Registra fontes descobertas sem aprová-las automaticamente.")
    parser.add_argument("csv_file", type=Path)
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--schema", type=Path, default=Path(__file__).resolve().parents[1] / "database" / "001_initial_schema.sql")
    args = parser.parse_args()
    report = register_sources(args.csv_file, args.db, args.schema)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report["status"] == "completed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
