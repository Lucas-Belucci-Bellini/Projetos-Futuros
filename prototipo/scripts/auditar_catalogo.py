#!/usr/bin/env python3
"""Read-only integrity audit for the construction-materials catalog."""
from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path


def audit_catalog(db_path: Path, stale_days: int = 30) -> dict:
    if not db_path.is_file():
        return {"database": str(db_path), "status": "missing_database", "findings": [
            {"severity": "error", "code": "database_missing", "count": 1,
             "message": "Banco não encontrado."}
        ]}

    findings: list[dict] = []
    try:
        connection = sqlite3.connect(f"file:{db_path.resolve()}?mode=ro", uri=True)
        connection.row_factory = sqlite3.Row

        # Foreign-key violations are errors, not guesses that the auditor should repair.
        fk_rows = list(connection.execute("PRAGMA foreign_key_check"))
        if fk_rows:
            findings.append({"severity": "error", "code": "foreign_key_violation",
                             "count": len(fk_rows), "message": "Há referências que violam chaves estrangeiras."})

        checks = [
            ("products_from_unapproved_sources",
             """SELECT COUNT(*) FROM source_products p JOIN sources s ON s.id=p.source_id
                WHERE s.permission_status <> 'approved'""",
             "error", "Produtos registrados com fonte que não está aprovada."),
            ("offers_from_unapproved_sources",
             """SELECT COUNT(*) FROM offers o JOIN sources s ON s.id=o.source_id
                WHERE s.permission_status <> 'approved'""",
             "error", "Ofertas registradas com fonte que não está aprovada."),
            ("offers_for_unconfirmed_stores",
             """SELECT COUNT(*) FROM offers o JOIN stores st ON st.id=o.store_id
                WHERE st.verification_status <> 'confirmed'""",
             "warning", "Ofertas associadas a lojas que ainda não foram confirmadas."),
            ("offers_with_old_observation",
             """SELECT COUNT(*) FROM offers
                WHERE observed_at IS NOT NULL AND TRIM(observed_at) <> ''
                  AND julianday('now') - julianday(observed_at) > ?""",
             "warning", f"Ofertas observadas há mais de {stale_days} dias."),
            ("offers_missing_price",
             """SELECT COUNT(*) FROM offers WHERE price_amount IS NULL""",
             "info", "Ofertas sem preço numérico; podem representar disponibilidade ou produto sem preço publicado."),
        ]
        for code, query, severity, message in checks:
            params = (stale_days,) if code == "offers_with_old_observation" else ()
            count = int(connection.execute(query, params).fetchone()[0])
            if count:
                findings.append({"severity": severity, "code": code, "count": count, "message": message})

        counts = {}
        for table in ("stores", "sources", "source_products", "offers", "import_batches", "import_errors"):
            exists = connection.execute(
                "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (table,)
            ).fetchone()
            counts[table] = int(connection.execute(f'SELECT COUNT(*) FROM "{table}"').fetchone()[0]) if exists else 0

        connection.close()
        has_errors = any(item["severity"] == "error" for item in findings)
        return {"database": str(db_path), "status": "issues_found" if findings else "ok",
                "has_errors": has_errors, "stale_days": stale_days, "counts": counts, "findings": findings}
    except sqlite3.Error as exc:
        return {"database": str(db_path), "status": "database_error", "has_errors": True,
                "findings": [{"severity": "error", "code": "database_error", "count": 1,
                              "message": str(exc)}]}


def main() -> int:
    parser = argparse.ArgumentParser(description="Audita integridade do catálogo sem alterar o banco.")
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--stale-days", type=int, default=30)
    args = parser.parse_args()
    if args.stale_days < 1:
        parser.error("--stale-days deve ser maior que zero")
    report = audit_catalog(args.db, args.stale_days)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 1 if report.get("has_errors") or report["status"] in {"missing_database", "database_error"} else 0


if __name__ == "__main__":
    raise SystemExit(main())
