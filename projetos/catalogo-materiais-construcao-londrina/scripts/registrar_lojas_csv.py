#!/usr/bin/env python3
"""Register local stores as pending until their details are manually verified."""
from __future__ import annotations

import argparse
import csv
import json
import sqlite3
from pathlib import Path
from urllib.parse import urlparse

REQUIRED = {"id", "name", "official_url", "city", "state", "address", "verification_status"}


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
            if row.get("source_url") and not valid_url(row["source_url"]):
                row_errors.append(f"linha {line}: source_url inválida")
            if row.get("verification_status") not in {"pending", "stale"}:
                row_errors.append(
                    f"linha {line}: status inválido; CSV não pode marcar loja como confirmada automaticamente"
                )
            if row_errors:
                errors.extend(row_errors)
            else:
                rows.append(row)
    return rows, errors


def register_stores(csv_path: Path, db_path: Path, schema_path: Path) -> dict:
    rows, errors = load_rows(csv_path)
    report = {"file": str(csv_path), "rows_read": len(rows) + len(errors), "accepted_rows": len(rows),
              "inserted_rows": 0, "updated_rows": 0, "errors": errors, "status": "rejected"}
    if errors or not rows:
        if not errors:
            report["errors"].append("CSV não contém lojas.")
        return report
    try:
        connection = sqlite3.connect(db_path)
        try:
            connection.executescript(Path(schema_path).read_text(encoding="utf-8"))
            for row in rows:
                exists = connection.execute("SELECT 1 FROM stores WHERE id=?", (row["id"],)).fetchone()
                connection.execute(
                    """INSERT INTO stores
                       (id, name, official_url, city, state, address, phone_public,
                        verification_status, source_url, verified_at)
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                       ON CONFLICT(id) DO UPDATE SET
                         name=excluded.name, official_url=excluded.official_url,
                         city=excluded.city, state=excluded.state, address=excluded.address,
                         phone_public=excluded.phone_public, verification_status='pending',
                         source_url=excluded.source_url, verified_at=NULL,
                         updated_at=CURRENT_TIMESTAMP""",
                    (row["id"], row["name"], row["official_url"], row["city"], row["state"],
                     row["address"], row.get("phone_public") or None, row["verification_status"],
                     row.get("source_url") or None, row.get("verified_at") or None),
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
    parser = argparse.ArgumentParser(description="Registra lojas locais em estado pendente de verificação.")
    parser.add_argument("csv_file", type=Path)
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--schema", type=Path, default=Path(__file__).resolve().parents[1] / "database" / "001_initial_schema.sql")
    args = parser.parse_args()
    report = register_stores(args.csv_file, args.db, args.schema)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report["status"] == "completed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
