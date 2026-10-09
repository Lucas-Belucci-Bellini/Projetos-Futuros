#!/usr/bin/env python3
"""Import store offers from an authorized CSV into SQLite."""
from __future__ import annotations

import argparse
import csv
import json
import re
import sqlite3
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation
from pathlib import Path
from urllib.parse import urlparse

REQUIRED = {
    "source_id", "store_id", "currency", "price_type", "availability",
    "fulfillment", "offer_url", "observed_at",
}
PRICE_TYPES = {"regular", "sale", "cash", "installment", "conditional", "unknown"}
AVAILABILITY = {"in_stock", "out_of_stock", "unknown", "not_applicable"}
FULFILLMENT = {"online", "store", "pickup", "delivery", "unknown"}
CURRENCY_RE = re.compile(r"^[A-Z]{3}$")


def valid_url(value: str) -> bool:
    try:
        parsed = urlparse(value.strip())
        return parsed.scheme.lower() in {"http", "https"} and bool(parsed.netloc) and "." in parsed.netloc
    except ValueError:
        return False


def valid_timestamp(value: str) -> bool:
    try:
        datetime.fromisoformat(value.strip().replace("Z", "+00:00"))
        return bool(value.strip())
    except ValueError:
        return False


def validate_rows(csv_path: Path) -> tuple[list[dict[str, str]], list[str]]:
    errors: list[str] = []
    rows: list[dict[str, str]] = []
    try:
        with csv_path.open("r", encoding="utf-8-sig", newline="") as handle:
            reader = csv.DictReader(handle)
            if not reader.fieldnames:
                return [], ["arquivo vazio ou sem cabeçalho"]
            headers = {header.strip() for header in reader.fieldnames if header}
            missing = sorted(REQUIRED - headers)
            if missing:
                return [], ["colunas obrigatórias ausentes: " + ", ".join(missing)]
            for line, raw in enumerate(reader, start=2):
                row = {(key or "").strip(): (value or "").strip() for key, value in raw.items() if key is not None}
                row_errors = []
                for field in REQUIRED:
                    if not row.get(field):
                        row_errors.append(f"linha {line}: campo obrigatório vazio: {field}")
                if row.get("offer_url") and not valid_url(row["offer_url"]):
                    row_errors.append(f"linha {line}: offer_url inválida")
                if row.get("observed_at") and not valid_timestamp(row["observed_at"]):
                    row_errors.append(f"linha {line}: observed_at inválido")
                if row.get("expires_at") and not valid_timestamp(row["expires_at"]):
                    row_errors.append(f"linha {line}: expires_at inválido")
                if row.get("currency") and not CURRENCY_RE.fullmatch(row["currency"]):
                    row_errors.append(f"linha {line}: currency deve ser um código de três letras maiúsculas")
                if row.get("price_type") and row["price_type"] not in PRICE_TYPES:
                    row_errors.append(f"linha {line}: price_type não reconhecido")
                if row.get("availability") and row["availability"] not in AVAILABILITY:
                    row_errors.append(f"linha {line}: availability não reconhecida")
                if row.get("fulfillment") and row["fulfillment"] not in FULFILLMENT:
                    row_errors.append(f"linha {line}: fulfillment não reconhecido")
                if row.get("price_amount"):
                    try:
                        price = Decimal(row["price_amount"])
                        if not price.is_finite() or price <= 0:
                            row_errors.append(f"linha {line}: price_amount deve ser positivo; deixe vazio se desconhecido")
                    except InvalidOperation:
                        row_errors.append(f"linha {line}: price_amount deve usar decimal com ponto, por exemplo 129.90")
                if row_errors:
                    errors.extend(row_errors)
                else:
                    rows.append(row)
    except (OSError, UnicodeDecodeError, csv.Error) as exc:
        errors.append(f"falha lendo CSV: {exc}")
    return rows, errors


def import_offers(csv_path: Path, db_path: Path, schema_path: Path, source_id: str, dry_run: bool = False) -> dict:
    rows, errors = validate_rows(csv_path)
    report = {
        "file": str(csv_path), "source_id": source_id, "dry_run": dry_run,
        "rows_read": len(rows) + len(errors), "accepted_rows": len(rows),
        "inserted_rows": 0, "updated_rows": 0, "unchanged_rows": 0,
        "rejected_rows": len(errors), "errors": errors, "status": "rejected",
    }
    if errors:
        report["errors"].append("Lote rejeitado: corrija todas as linhas antes de importar.")
        return report
    if not rows:
        report["errors"].append("CSV não contém ofertas para importar.")
        return report
    if any(row.get("source_id") != source_id for row in rows):
        report["errors"].append("source_id do CSV difere do --source-id.")
        return report

    try:
        connection = sqlite3.connect(db_path)
        connection.row_factory = sqlite3.Row
        try:
            connection.executescript(Path(schema_path).read_text(encoding="utf-8"))
            source = connection.execute(
                "SELECT permission_status FROM sources WHERE id=?", (source_id,)
            ).fetchone()
            if source is None or source["permission_status"] != "approved":
                report["errors"].append("Fonte inexistente ou não aprovada para importação.")
                return report

            prepared = []
            for index, row in enumerate(rows, start=2):
                store = connection.execute("SELECT id FROM stores WHERE id=?", (row["store_id"],)).fetchone()
                if store is None:
                    report["errors"].append(f"linha {index}: store_id não existe no banco: {row['store_id']}")
                    continue
                product_db_id = None
                external_product_id = row.get("source_product_id", "")
                if external_product_id:
                    product = connection.execute(
                        "SELECT id FROM source_products WHERE source_id=? AND source_product_id=?",
                        (source_id, external_product_id),
                    ).fetchone()
                    if product is None:
                        report["errors"].append(
                            f"linha {index}: source_product_id não encontrado para esta fonte: {external_product_id}"
                        )
                        continue
                    product_db_id = product["id"]
                prepared.append((row, product_db_id))

            if report["errors"]:
                report["status"] = "rejected"
                return report

            now = datetime.now(timezone.utc).isoformat()
            batch = connection.execute(
                """INSERT INTO import_batches
                   (source_id, file_name, started_at, rows_read, accepted_rows, status)
                   VALUES (?, ?, ?, ?, ?, ?)""",
                (source_id, csv_path.name, now, len(rows), len(rows), "dry_run" if dry_run else "running"),
            )
            batch_id = batch.lastrowid

            for row, product_db_id in prepared:
                offer_id = row.get("source_offer_id", "")
                if offer_id:
                    existing = connection.execute(
                        "SELECT * FROM offers WHERE source_id=? AND source_offer_id=?",
                        (source_id, offer_id),
                    ).fetchone()
                else:
                    existing = connection.execute(
                        "SELECT * FROM offers WHERE source_id=? AND store_id=? AND offer_url=? AND (source_offer_id IS NULL OR source_offer_id='')",
                        (source_id, row["store_id"], row["offer_url"]),
                    ).fetchone()

                price = Decimal(row["price_amount"]) if row.get("price_amount") else None
                values = (
                    row["store_id"], row.get("source_offer_id") or None, product_db_id,
                    str(price) if price is not None else None, row["currency"], row["price_type"],
                    row["availability"], row["fulfillment"], row.get("conditions") or None,
                    row["offer_url"], row["observed_at"], row.get("expires_at") or None,
                )
                fields = (
                    "store_id", "source_offer_id", "source_product_id", "price_amount", "currency",
                    "price_type", "availability", "fulfillment", "conditions", "offer_url",
                    "observed_at", "expires_at",
                )
                if existing:
                    old_values = tuple(existing[field] for field in fields)
                    comparable = tuple(float(v) if field == "price_amount" and v is not None else v for field, v in zip(fields, old_values))
                    new_comparable = tuple(float(v) if field == "price_amount" and v is not None else v for field, v in zip(fields, values))
                    if comparable == new_comparable:
                        report["unchanged_rows"] += 1
                        continue
                    assignments = ", ".join(f"{field}=?" for field in fields)
                    connection.execute(
                        f"UPDATE offers SET {assignments}, status='pending' WHERE id=?",
                        list(values) + [existing["id"]],
                    )
                    report["updated_rows"] += 1
                else:
                    connection.execute(
                        """INSERT INTO offers
                        (source_id, store_id, source_offer_id, source_product_id, price_amount,
                         currency, price_type, availability, fulfillment, conditions, offer_url,
                         observed_at, expires_at, status)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending')""",
                        (source_id,) + values,
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
    except (OSError, sqlite3.Error) as exc:
        report["status"] = "failed"
        report["errors"].append(str(exc))
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description="Importa ofertas autorizadas de um CSV para SQLite.")
    parser.add_argument("csv_file", type=Path)
    parser.add_argument("--db", type=Path, default=Path("catalogo-materiais.sqlite3"))
    parser.add_argument("--source-id", required=True)
    parser.add_argument("--schema", type=Path, default=Path(__file__).resolve().parents[1] / "database" / "001_initial_schema.sql")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    report = import_offers(args.csv_file, args.db, args.schema, args.source_id, args.dry_run)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report["status"] in {"completed", "dry_run"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
