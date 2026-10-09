#!/usr/bin/env python3
"""Generate a read-only JSON overview of the catalog database."""
from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path


def table_exists(connection: sqlite3.Connection, table: str) -> bool:
    return connection.execute(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (table,)
    ).fetchone() is not None


def count(connection: sqlite3.Connection, table: str) -> int:
    if not table_exists(connection, table):
        return 0
    return int(connection.execute(f'SELECT COUNT(*) FROM "{table}"').fetchone()[0])


def build_report(db_path: Path) -> dict:
    if not db_path.is_file():
        return {
            "database": str(db_path),
            "status": "missing_database",
            "message": "Banco não encontrado. Crie o banco usando database/001_initial_schema.sql.",
        }

    report = {
        "database": str(db_path),
        "status": "ok",
        "counts": {},
        "source_status": {},
        "top_categories": [],
        "recent_imports": [],
        "quality_warnings": [],
    }
    try:
        connection = sqlite3.connect(f"file:{db_path.resolve()}?mode=ro", uri=True)
        connection.row_factory = sqlite3.Row
        with connection:
            for table in ("stores", "sources", "source_products", "offers", "import_batches", "import_errors"):
                report["counts"][table] = count(connection, table)

            if table_exists(connection, "sources"):
                for row in connection.execute(
                    "SELECT permission_status, COUNT(*) AS total FROM sources GROUP BY permission_status"
                ):
                    report["source_status"][row["permission_status"]] = row["total"]

            if table_exists(connection, "source_products"):
                report["top_categories"] = [
                    dict(row) for row in connection.execute(
                        """SELECT category_path, COUNT(*) AS total
                           FROM source_products
                           GROUP BY category_path
                           ORDER BY total DESC, category_path ASC
                           LIMIT 20"""
                    )
                ]
                missing_source = connection.execute(
                    """SELECT COUNT(*) FROM source_products
                       WHERE product_url IS NULL OR TRIM(product_url)=''
                          OR observed_at IS NULL OR TRIM(observed_at)=''"""
                ).fetchone()[0]
                if missing_source:
                    report["quality_warnings"].append(
                        f"{missing_source} produto(s) sem URL ou data de observação."
                    )

            if table_exists(connection, "offers"):
                missing_price_date = connection.execute(
                    """SELECT COUNT(*) FROM offers
                       WHERE price_amount IS NOT NULL
                         AND (observed_at IS NULL OR TRIM(observed_at)='')"""
                ).fetchone()[0]
                if missing_price_date:
                    report["quality_warnings"].append(
                        f"{missing_price_date} oferta(s) com preço sem data de observação."
                    )

            if table_exists(connection, "import_batches"):
                report["recent_imports"] = [
                    dict(row) for row in connection.execute(
                        """SELECT id, source_id, file_name, started_at, finished_at,
                                  rows_read, inserted_rows, updated_rows, rejected_rows, status
                           FROM import_batches
                           ORDER BY id DESC LIMIT 10"""
                    )
                ]
        connection.close()
    except sqlite3.Error as exc:
        report["status"] = "database_error"
        report["message"] = str(exc)
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description="Gera relatório somente de leitura do catálogo SQLite.")
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--output", type=Path, help="salvar relatório JSON neste caminho")
    args = parser.parse_args()
    report = build_report(args.db)
    rendered = json.dumps(report, ensure_ascii=False, indent=2)
    if args.output:
        args.output.write_text(rendered + "\n", encoding="utf-8")
    print(rendered)
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
