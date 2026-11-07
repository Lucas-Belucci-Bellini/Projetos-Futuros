#!/usr/bin/env python3
"""Import product CSV rows into SQLite after structural validation.

The source must already exist in the sources table with permission_status='approved'.
The import is transactional and idempotent by source product code, or source URL
when the source has no stable product code. It does not fetch websites.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path

try:
    from validar_catalogo_csv import validate_file
except ImportError:  # supports package-style imports in tests
    from scripts.validar_catalogo_csv import validate_file

PRODUCT_FIELDS = (
    "source_product_id", "product_name", "brand", "category_path",
    "manufacturer_code", "gtin", "variant", "sale_unit", "package_quantity",
    "package_unit", "short_description", "product_url", "image_url",
    "observed_at", "reuse_permission",
)


def row_hash(row: dict[str, str]) -> str:
    canonical = json.dumps(
        {key: (row.get(key) or "").strip() for key in PRODUCT_FIELDS},
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    )
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def initialize_database(connection: sqlite3.Connection, schema_path: Path) -> None:
    connection.executescript(schema_path.read_text(encoding="utf-8"))


def import_csv(csv_path: Path, db_path: Path, schema_path: Path, source_id: str, dry_run: bool = False) -> dict:
    validation = validate_file(csv_path)
    report = {
        "file": str(csv_path),
        "source_id": source_id,
        "dry_run": dry_run,
        "validation": validation,
        "inserted_rows": 0,
        "updated_rows": 0,
        "skipped_unchanged_rows": 0,
        "status": "rejected",
        "errors": [],
    }
    if not validation["valid_for_import"]:
        report["errors"].append("CSV reprovado na validação; nenhuma linha foi importada.")
        return report

    # The command-line source ID must match every row, preventing accidental
    # attribution of a file to the wrong store or permission record.
    with csv_path.open("r", encoding="utf-8-sig", newline="") as handle:
        preview_rows = list(csv.DictReader(handle))
    mismatched_rows = [
        index for index, row in enumerate(preview_rows, start=2)
        if (row.get("source_id") or "").strip() != source_id
    ]
    if mismatched_rows:
        report["errors"].append(
            "source_id do CSV difere do --source-id nas linhas: "
            + ", ".join(map(str, mismatched_rows[:50]))
        )
        report["status"] = "rejected"
        return report

    schema_path = Path(schema_path)
    db_path = Path(db_path)
    if not schema_path.is_file():
        report["errors"].append(f"Schema SQLite não encontrado: {schema_path}")
        return report

    try:
        with csv_path.open("r", encoding="utf-8-sig", newline="") as handle:
            rows = list(csv.DictReader(handle))
        connection = sqlite3.connect(db_path)
        connection.row_factory = sqlite3.Row
        try:
            initialize_database(connection, schema_path)
            source = connection.execute(
                "SELECT id, permission_status FROM sources WHERE id = ?", (source_id,)
            ).fetchone()
            if source is None:
                report["errors"].append(
                    "Fonte não cadastrada no banco. Registre e aprove a fonte antes de importar."
                )
                return report
            if source["permission_status"] != "approved":
                report["errors"].append(
                    "Fonte não está com permission_status='approved'; importação bloqueada."
                )
                return report

            now = datetime.now(timezone.utc).isoformat()
            batch_cursor = connection.execute(
                """INSERT INTO import_batches
                   (source_id, file_name, started_at, rows_read, accepted_rows, status)
                   VALUES (?, ?, ?, ?, ?, ?)""",
                (source_id, csv_path.name, now, len(rows), len(rows), "dry_run" if dry_run else "running"),
            )
            batch_id = batch_cursor.lastrowid
            for raw in rows:
                row = {(key or "").strip(): (value or "").strip() for key, value in raw.items()}
                digest = row_hash(row)
                source_code = row.get("source_product_id", "")
                if source_code:
                    existing = connection.execute(
                        "SELECT id, content_hash FROM source_products WHERE source_id=? AND source_product_id=?",
                        (source_id, source_code),
                    ).fetchone()
                else:
                    existing = connection.execute(
                        "SELECT id, content_hash FROM source_products WHERE source_id=? AND product_url=? AND (source_product_id IS NULL OR source_product_id='')",
                        (source_id, row["product_url"]),
                    ).fetchone()

                if existing and existing["content_hash"] == digest:
                    connection.execute(
                        "UPDATE source_products SET last_seen_at=?, last_import_batch_id=? WHERE id=?",
                        (now, batch_id, existing["id"]),
                    )
                    report["skipped_unchanged_rows"] += 1
                    continue

                values = [source_id] + [row.get(field, "") or None for field in PRODUCT_FIELDS]
                if existing:
                    assignments = ", ".join(f"{field}=?" for field in PRODUCT_FIELDS)
                    connection.execute(
                        f"""UPDATE source_products SET {assignments}, content_hash=?,
                            last_seen_at=?, last_import_batch_id=? WHERE id=?""",
                        [row.get(field, "") or None for field in PRODUCT_FIELDS]
                        + [digest, now, batch_id, existing["id"]],
                    )
                    report["updated_rows"] += 1
                else:
                    connection.execute(
                        """INSERT INTO source_products
                        (source_id, source_product_id, product_name, brand, category_path,
                         manufacturer_code, gtin, variant, sale_unit, package_quantity,
                         package_unit, short_description, product_url, image_url,
                         observed_at, reuse_permission, content_hash, last_seen_at, last_import_batch_id)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                        values + [digest, now, batch_id],
                    )
                    report["inserted_rows"] += 1

            report["status"] = "dry_run" if dry_run else "completed"
            connection.execute(
                """UPDATE import_batches SET finished_at=?, inserted_rows=?, updated_rows=?,
                   status=?, report_json=? WHERE id=?""",
                (now, report["inserted_rows"], report["updated_rows"], report["status"],
                 json.dumps(report, ensure_ascii=False), batch_id),
            )
            if dry_run:
                connection.rollback()
            else:
                connection.commit()
        except Exception:
            connection.rollback()
            raise
        finally:
            connection.close()
    except (OSError, sqlite3.Error, csv.Error) as exc:
        report["status"] = "failed"
        report["errors"].append(str(exc))

    return report


def main() -> int:
    parser = argparse.ArgumentParser(description="Importa produtos CSV autorizados para SQLite.")
    parser.add_argument("csv_file", type=Path)
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--source-id", required=True, help="ID da fonte já aprovada no banco")
    parser.add_argument("--schema", type=Path, default=Path(__file__).resolve().parents[1] / "database" / "001_initial_schema.sql")
    parser.add_argument("--dry-run", action="store_true", help="simula a importação e reverte as alterações ao final")
    args = parser.parse_args()

    report = import_csv(args.csv_file, args.db, args.schema, args.source_id, args.dry_run)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report["status"] in {"completed", "dry_run"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
