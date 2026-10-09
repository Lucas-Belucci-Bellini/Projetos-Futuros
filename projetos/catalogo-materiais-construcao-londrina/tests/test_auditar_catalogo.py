import sqlite3
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from auditar_catalogo import audit_catalog

SCHEMA = ROOT / "database" / "001_initial_schema.sql"


class CatalogAuditTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.db = self.root / "catalog.sqlite3"
        with sqlite3.connect(self.db) as connection:
            connection.executescript(SCHEMA.read_text(encoding="utf-8"))

    def tearDown(self):
        self.temp.cleanup()

    def test_empty_schema_has_no_findings(self):
        report = audit_catalog(self.db)
        self.assertEqual(report["status"], "ok")
        self.assertEqual(report["findings"], [])

    def test_flags_offer_from_unapproved_source_and_unconfirmed_store(self):
        with sqlite3.connect(self.db) as connection:
            connection.execute(
                """INSERT INTO sources
                   (id, organization_name, official_url, source_type, access_method, permission_status)
                   VALUES ('source-demo', 'Fonte demo', 'https://example.com', 'site', 'manual', 'discovered')"""
            )
            connection.execute(
                """INSERT INTO stores
                   (id, name, official_url, city, state, address, verification_status)
                   VALUES ('store-demo', 'Loja demo', 'https://example.com', 'Londrina', 'PR', 'Endereço demo', 'pending')"""
            )
            connection.execute(
                """INSERT INTO offers (source_id, store_id, offer_url, observed_at)
                   VALUES ('source-demo', 'store-demo', 'https://example.com/item', '2026-10-01T10:00:00+00:00')"""
            )
            connection.commit()

        report = audit_catalog(self.db)
        codes = {finding["code"]: finding["count"] for finding in report["findings"]}
        self.assertEqual(report["status"], "issues_found")
        self.assertTrue(report["has_errors"])
        self.assertEqual(codes["offers_from_unapproved_sources"], 1)
        self.assertEqual(codes["offers_for_unconfirmed_stores"], 1)

    def test_missing_database_is_reported(self):
        report = audit_catalog(self.root / "missing.sqlite3")
        self.assertEqual(report["status"], "missing_database")


if __name__ == "__main__":
    unittest.main()
