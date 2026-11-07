#!/usr/bin/env python3
"""Validate a product-catalog CSV without importing or publishing its data.

Usage:
    python scripts/validar_catalogo_csv.py products.csv
    python scripts/validar_catalogo_csv.py products.csv --report report.json

Only rows with explicit authorization for the intended data reuse are accepted.
This tool validates structure and basic fields; it does not prove that source
permissions are legally sufficient or that product identity is correct.
"""
from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from datetime import datetime
from pathlib import Path
from urllib.parse import urlparse

REQUIRED_COLUMNS = {
    "source_id",
    "product_name",
    "category_path",
    "sale_unit",
    "product_url",
    "observed_at",
    "reuse_permission",
}
ALLOWED_PERMISSION = {"authorized"}
ALLOWED_URL_SCHEMES = {"https", "http"}
MAX_FIELD_LENGTH = 4000
PRICE_LIKE_PATTERN = re.compile(r"^\s*-\d+(?:[.,]\d+)?\s*$")


def validate_url(value: str) -> bool:
    try:
        parsed = urlparse(value.strip())
        return (
            parsed.scheme.lower() in ALLOWED_URL_SCHEMES
            and bool(parsed.netloc)
            and "." in parsed.netloc
        )
    except ValueError:
        return False


def validate_timestamp(value: str) -> bool:
    candidate = value.strip()
    if not candidate:
        return False
    try:
        datetime.fromisoformat(candidate.replace("Z", "+00:00"))
        return True
    except ValueError:
        return False


def validate_row(row: dict[str, str], line_number: int) -> list[str]:
    errors: list[str] = []

    for field in sorted(REQUIRED_COLUMNS):
        if not (row.get(field) or "").strip():
            errors.append(f"linha {line_number}: campo obrigatório vazio: {field}")

    for field, value in row.items():
        if value and len(value) > MAX_FIELD_LENGTH:
            errors.append(
                f"linha {line_number}: campo excede {MAX_FIELD_LENGTH} caracteres: {field}"
            )

    if (row.get("product_url") or "").strip() and not validate_url(row["product_url"]):
        errors.append(f"linha {line_number}: product_url deve ser uma URL HTTP(S) válida")

    if (row.get("observed_at") or "").strip() and not validate_timestamp(row["observed_at"]):
        errors.append(f"linha {line_number}: observed_at deve ser uma data ISO 8601 válida")

    permission = (row.get("reuse_permission") or "").strip().lower()
    if permission not in ALLOWED_PERMISSION:
        errors.append(
            f"linha {line_number}: reuse_permission precisa ser 'authorized' "
            "para aceitar dados neste pipeline"
        )

    if (row.get("product_name") or "").strip() and PRICE_LIKE_PATTERN.match(row["product_name"]):
        errors.append(f"linha {line_number}: product_name parece conter apenas um valor numérico negativo")

    return errors


def validate_file(csv_path: Path) -> dict:
    report = {
        "file": str(csv_path),
        "rows_read": 0,
        "accepted_rows": 0,
        "rejected_rows": 0,
        "errors": [],
        "missing_columns": [],
        "valid_for_import": False,
        "notice": (
            "Validação estrutural inicial; não prova autorização legal, "
            "exatidão comercial nem identidade única do produto."
        ),
    }

    try:
        with csv_path.open("r", encoding="utf-8-sig", newline="") as handle:
            reader = csv.DictReader(handle)
            if not reader.fieldnames:
                report["errors"].append("arquivo vazio ou sem cabeçalho")
                return report

            normalized_headers = [header.strip() for header in reader.fieldnames if header]
            report["missing_columns"] = sorted(REQUIRED_COLUMNS - set(normalized_headers))
            if report["missing_columns"]:
                report["errors"].append(
                    "colunas obrigatórias ausentes: " + ", ".join(report["missing_columns"])
                )
                return report

            for line_number, raw_row in enumerate(reader, start=2):
                report["rows_read"] += 1
                row = {
                    (key or "").strip(): (value or "").strip()
                    for key, value in raw_row.items()
                    if key is not None
                }
                row_errors = validate_row(row, line_number)
                if row_errors:
                    report["rejected_rows"] += 1
                    report["errors"].extend(row_errors)
                else:
                    report["accepted_rows"] += 1

    except UnicodeDecodeError:
        report["errors"].append("arquivo não está codificado em UTF-8")
    except OSError as exc:
        report["errors"].append(f"não foi possível ler o arquivo: {exc}")

    report["valid_for_import"] = (
        report["rows_read"] > 0
        and report["rejected_rows"] == 0
        and not report["missing_columns"]
        and not report["errors"]
    )
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description="Valida um CSV de produtos antes da importação.")
    parser.add_argument("csv_file", type=Path, help="caminho do CSV")
    parser.add_argument("--report", type=Path, help="caminho opcional para salvar o relatório JSON")
    args = parser.parse_args()

    report = validate_file(args.csv_file)
    rendered = json.dumps(report, ensure_ascii=False, indent=2)

    if args.report:
        try:
            args.report.write_text(rendered + "\n", encoding="utf-8")
        except OSError as exc:
            print(f"Erro ao gravar relatório: {exc}", file=sys.stderr)
            return 2

    print(rendered)
    return 0 if report["valid_for_import"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
