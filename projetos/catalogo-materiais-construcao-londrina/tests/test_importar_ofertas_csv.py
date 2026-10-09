import csv
import sqlite3
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from importar_ofertas_csv import import_offers

SCHEMA = ROOT / "database" / "001_initial_schema.sql"
FIELDS = [
    "source_id", "source_offer_id", "store_id", "source_product_id", "price_amount",
    "currency", "price_type", "availability", "fulfillment", "conditions",
    "offer_url", "observed_at", "expires_at",
]
ROW = {
    "source_id": "source-demo", "source_offer_id": "OFFER-01", "store_id": "store-demo",
    "source_product_id": "SKU-01", "price_amount": "129.90", "currency": "BRL",
    "price_type": "regular", "availability": "unknown", "fulfillment": "online",
    "conditions": "", "offer_url": "https://example.com/oferta",
    "observed_at": "2026-10-09T12:00:00-03:00", "expires_at": "",
}


class ImportOffersTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.db = self.root / "catalog.sqlite3"
        self.csv_path = self.root / "offers.csv"
        with sqlite3.connect(self.db) as conn:
            conn.executescript(SCHEMA.read_text(encoding="utf-8"))
            conn.execute(
                """INSERT INTO sources
                   (id, organization_name, official_url, source_type, access_method, permission_status)
                   VALUES (?, ?, ?, ?, ?, ?)""",
                ("source-demo", "Fonte demonstrativa", "https://example.com",
                 "authorized_file", "arquivo de teste", "approved"),
            )
            conn.execute(
                "INSERT INTO stores (id, name) VALUES (?, ?)",
                ("store-demo", "Loja de teste"),
            )
            conn.execute(
                """INSERT INTO source_products
                   (source_id, source_product_id, product_name, category_path, sale_unit,
                    product_url, observed_at, reuse_permission, content_hash)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                ("source-demo", "SKU-01", "Produto teste", "Materiais > Exemplo", "unidade",
                 "https://example.com/produto", "2026-10-09T12:00:00-03:00", "authorized", "hash"),
            )
        self.write_csv(ROW)

    def write_csv(self, row):
        with self.csv_path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=FIELDS)
            writer.writeheader()
            writer.writerow(row)

    def tearDown(self):
        self.temp.cleanup()

    def test_imports_offer(self):
        report = import_offers(self.csv_path, self.db, SCHEMA, "source-demo")
        self.assertEqual(report["status"], "completed")
        self.assertEqual(report["inserted_rows"], 1)
        with sqlite3.connect(self.db) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM offers").fetchone()[0], 1)

    def test_reimport_does_not_duplicate_offer(self):
        import_offers(self.csv_path, self.db, SCHEMA, "source-demo")
        report = import_offers(self.csv_path, self.db, SCHEMA, "source-demo")
        self.assertEqual(report["inserted_rows"], 0)
        self.assertEqual(report["unchanged_rows"], 1)
        with sqlite3.connect(self.db) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM offers").fetchone()[0], 1)

    def test_rejects_unknown_store(self):
        self.write_csv(dict(ROW, store_id="missing-store"))
        report = import_offers(self.csv_path, self.db, SCHEMA, "source-demo")
        self.assertEqual(report["status"], "rejected")
        self.assertTrue(any("store_id não existe" in error for error in report["errors"]))

    def test_rejects_zero_price(self):
        self.write_csv(dict(ROW, price_amount="0"))
        report = import_offers(self.csv_path, self.db, SCHEMA, "source-demo")
        self.assertEqual(report["status"], "rejected")

    def test_dry_run_does_not_persist_offer(self):
        report = import_offers(self.csv_path, self.db, SCHEMA, "source-demo", dry_run=True)
        self.assertEqual(report["status"], "dry_run")
        with sqlite3.connect(self.db) as conn:
            self.assertEqual(conn.execute("SELECT COUNT(*) FROM offers").fetchone()[0], 0)


if __name__ == "__main__":
    unittest.main()
