import csv
import sqlite3
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from registrar_fontes_csv import register_sources, load_rows as load_sources
from registrar_lojas_csv import register_stores, load_rows as load_stores

SCHEMA = ROOT / "database" / "001_initial_schema.sql"
SOURCE_FIELDS = [
    "source_id", "organization_name", "official_url", "product_listing_url", "source_type",
    "geographic_scope", "access_method", "permission_status", "permission_evidence_url",
    "permitted_fields", "media_permission", "expected_refresh", "last_checked_at", "notes",
]
SOURCE_ROW = {
    "source_id": "source-demo", "organization_name": "Fonte demonstrativa",
    "official_url": "https://example.com", "product_listing_url": "https://example.com/products",
    "source_type": "site", "geographic_scope": "Londrina-PR", "access_method": "manual review",
    "permission_status": "discovered", "permission_evidence_url": "", "permitted_fields": "",
    "media_permission": "not_assessed", "expected_refresh": "", "last_checked_at": "", "notes": "",
}
STORE_FIELDS = [
    "id", "name", "official_url", "city", "state", "address", "phone_public",
    "verification_status", "source_url", "verified_at",
]
STORE_ROW = {
    "id": "store-demo", "name": "Loja demonstrativa", "official_url": "https://example.com",
    "city": "Londrina", "state": "PR", "address": "Endereço de teste", "phone_public": "",
    "verification_status": "pending", "source_url": "https://example.com/stores", "verified_at": "",
}


class RegistryImporterTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.db = self.root / "catalog.sqlite3"
        self.sources_csv = self.root / "sources.csv"
        self.stores_csv = self.root / "stores.csv"
        with sqlite3.connect(self.db) as connection:
            connection.executescript(SCHEMA.read_text(encoding="utf-8"))
        self.write_csv(self.sources_csv, SOURCE_FIELDS, SOURCE_ROW)
        self.write_csv(self.stores_csv, STORE_FIELDS, STORE_ROW)

    def tearDown(self):
        self.temp.cleanup()

    @staticmethod
    def write_csv(path, fields, row):
        with path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=fields)
            writer.writeheader()
            writer.writerow(row)

    def test_registers_source_without_approval(self):
        report = register_sources(self.sources_csv, self.db, SCHEMA)
        self.assertEqual(report["status"], "completed")
        with sqlite3.connect(self.db) as connection:
            status = connection.execute(
                "SELECT permission_status FROM sources WHERE id='source-demo'"
            ).fetchone()[0]
        self.assertEqual(status, "discovered")

    def test_source_csv_cannot_auto_approve(self):
        self.write_csv(self.sources_csv, SOURCE_FIELDS, dict(SOURCE_ROW, permission_status="approved"))
        rows, errors = load_sources(self.sources_csv)
        self.assertEqual(rows, [])
        self.assertTrue(any("não pode aprovar" in error for error in errors))

    def test_registers_store_as_pending(self):
        report = register_stores(self.stores_csv, self.db, SCHEMA)
        self.assertEqual(report["status"], "completed")
        with sqlite3.connect(self.db) as connection:
            status = connection.execute(
                "SELECT verification_status FROM stores WHERE id='store-demo'"
            ).fetchone()[0]
        self.assertEqual(status, "pending")

    def test_store_csv_cannot_auto_confirm(self):
        self.write_csv(self.stores_csv, STORE_FIELDS, dict(STORE_ROW, verification_status="confirmed"))
        rows, errors = load_stores(self.stores_csv)
        self.assertEqual(rows, [])
        self.assertTrue(any("não pode marcar loja como confirmada" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
