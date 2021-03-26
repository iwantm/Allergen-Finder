from tests.test_base import TestBase
from flask import url_for
from application import app, db
from application.models import Products


class TestApplication(TestBase):
    def test_get_product(self):
        res = self.client.get('/product/1')
        self.assertEqual(res.status_code, 200)
        self.assertIn(b'testProd', res.data)

    def test_add_product(self):
        res = self.client.get('/product/9300657007553')
        all_products = Products.query.all()
        this_product = Products.query.get('9300657007553')
        self.assertIn(this_product, all_products)
        self.assertEqual(res.status_code, 201)
        self.assertIn(b'Heinz', res.data)
