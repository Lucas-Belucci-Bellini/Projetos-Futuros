import sqlite3
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from relatorio_catalogo import build_report


class CatalogReportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.db = self.root / "catalog.sqlite3"
        schema = (ROOT / "database" / "001_initial_schema.sql").read_text(encoding="utf-8")
        with sqlite3.connect(self.db) as connection:
            connection.executescript(schema)
            connection.execute(
                """INSERT INTO sources
                   (id, organization_name, official_url, source_type, access_method, permission_status)
                   VALUES (?, ?, ?, ?, ?, ?)""",
                ("source-demo", "Fonte demonstrativa", "https://example.com",
                 "authorized_file", "arquivo de teste", "approved"),
            )

    def tearDown(self):
        self.temp.cleanup()

    def test_missing_database_is_reported(self):
        report = build_report(self.root / "missing.sqlite3")
        self.assertEqual(report["status"], "missing_database")

    def test_report_counts_product_and_category(self):
        with sqlite3.connect(self.db) as connection:
            connection.execute(
                """INSERT INTO source_products
                   (source_id, source_product_id, product_name, category_path, sale_unit,
                    product_url, observed_at, reuse_permission, content_hash)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                ("source-demo", "SKU-1", "Cimento demonstrativo",
                 "Materiais básicos > Cimento", "saco",
                 "https://example.com/cimento", "2026-10-09T12:00:00-03:00",
                 "authorized", "hash-demo"),
            )
        report = build_report(self.db)
        self.assertEqual(report["status"], "ok")
        self.assertEqual(report["counts"]["source_products"], 1)
        self.assertEqual(report["source_status"]["approved"], 1)
        self.assertEqual(report["top_categories"][0]["category_path"], "Materiais básicos > Cimento")

    def test_report_does_not_modify_database(self):
        before = self.db.stat().st_mtime_ns
        build_report(self.db)
        after = self.db.stat().st_mtime_ns
        self.assertEqual(before, after)


if __name__ == "__main__":
    unittest.main()
