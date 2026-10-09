import csv
import tempfile
import unittest
from pathlib import Path

from scripts.validar_catalogo_csv import validate_file, validate_row


VALID_ROW = {
    "source_id": "fonte-teste",
    "source_product_id": "SKU-001",
    "product_name": "Produto de demonstração",
    "brand": "Marca de demonstração",
    "category_path": "Materiais > Exemplo",
    "manufacturer_code": "",
    "gtin": "",
    "variant": "",
    "sale_unit": "unidade",
    "package_quantity": "",
    "package_unit": "",
    "short_description": "",
    "product_url": "https://example.com/produto",
    "image_url": "",
    "observed_at": "2026-10-09T12:00:00-03:00",
    "reuse_permission": "authorized",
}


class ValidateCatalogCsvTests(unittest.TestCase):
    def test_accepts_valid_row(self):
        self.assertEqual(validate_row(VALID_ROW, 2), [])

    def test_rejects_unapproved_reuse_permission(self):
        row = dict(VALID_ROW, reuse_permission="public_link_only")
        errors = validate_row(row, 2)
        self.assertTrue(any("reuse_permission" in error for error in errors))

    def test_rejects_invalid_url(self):
        row = dict(VALID_ROW, product_url="javascript:alert(1)")
        errors = validate_row(row, 2)
        self.assertTrue(any("product_url" in error for error in errors))

    def test_rejects_invalid_timestamp(self):
        row = dict(VALID_ROW, observed_at="yesterday")
        errors = validate_row(row, 2)
        self.assertTrue(any("observed_at" in error for error in errors))

    def test_reports_missing_columns(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "products.csv"
            path.write_text("product_name,product_url\nExample,https://example.com\n", encoding="utf-8")
            report = validate_file(path)
            self.assertFalse(report["valid_for_import"])
            self.assertIn("source_id", report["missing_columns"])

    def test_accepts_csv_with_valid_row(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "products.csv"
            with path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=list(VALID_ROW))
                writer.writeheader()
                writer.writerow(VALID_ROW)
            report = validate_file(path)
            self.assertEqual(report["rows_read"], 1)
            self.assertEqual(report["accepted_rows"], 1)
            self.assertEqual(report["rejected_rows"], 0)
            self.assertTrue(report["valid_for_import"])


if __name__ == "__main__":
    unittest.main()
