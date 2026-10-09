import csv
import sqlite3
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1] / "scripts"
import sys
sys.path.insert(0, str(SCRIPTS))
from importar_catalogo_csv import import_csv, initialize_database

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "database" / "001_initial_schema.sql"

FIELDS = [
    "source_id", "source_product_id", "product_name", "brand", "category_path",
    "manufacturer_code", "gtin", "variant", "sale_unit", "package_quantity",
    "package_unit", "short_description", "product_url", "image_url",
    "observed_at", "reuse_permission",
]
ROW = {
    "source_id": "source-demo",
    "source_product_id": "SKU-01",
    "product_name": "Produto demonstrativo",
    "brand": "Marca demonstrativa",
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


class ImportCatalogTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.csv_path = self.root / "products.csv"
        self.db_path = self.root / "catalog.sqlite3"
        with self.csv_path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=FIELDS)
            writer.writeheader()
            writer.writerow(ROW)
        with sqlite3.connect(self.db_path) as conn:
            initialize_database(conn, SCHEMA)
            conn.execute(
                "INSERT INTO sources (id, organization_name, official_url, source_type, access_method, permission_status) VALUES (?, ?, ?, ?, ?, ?)",
                ("source-demo", "Fonte demonstrativa", "https://example.com", "authorized_file", "arquivo de teste", "approved"),
            )

    def tearDown(self):
        self.temp.cleanup()

    def test_imports_one_valid_product(self):
        report = import_csv(self.csv_path, self.db_path, SCHEMA, "source-demo")
        self.assertEqual(report["status"], "completed")
        self.assertEqual(report["inserted_rows"], 1)
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM source_products").fetchone()[0], 1)

    def test_reimport_is_idempotent(self):
        import_csv(self.csv_path, self.db_path, SCHEMA, "source-demo")
        report = import_csv(self.csv_path, self.db_path, SCHEMA, "source-demo")
        self.assertEqual(report["inserted_rows"], 0)
        self.assertEqual(report["skipped_unchanged_rows"], 1)
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM source_products").fetchone()[0], 1)

    def test_blocks_unapproved_source(self):
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("UPDATE sources SET permission_status='under_review' WHERE id='source-demo'")
        report = import_csv(self.csv_path, self.db_path, SCHEMA, "source-demo")
        self.assertEqual(report["status"], "rejected")
        self.assertTrue(report["errors"])

    def test_dry_run_does_not_persist_product(self):
        report = import_csv(self.csv_path, self.db_path, SCHEMA, "source-demo", dry_run=True)
        self.assertEqual(report["status"], "dry_run")
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM source_products").fetchone()[0], 0)


if __name__ == "__main__":
    unittest.main()
