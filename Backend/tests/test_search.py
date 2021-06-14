from tests.test_base import TestBase
from flask import url_for
from api import app, db
from api.models import Products
from unittest.mock import patch


class TestApplication(TestBase):
    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_get_product(self, mock_jwt_required):
        res = self.client.get('/product/1')
        self.assertEqual(res.status_code, 200)
        self.assertIn(b'testProd', res.data)

    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_add_product(self, mock_jwt_required):
        res = self.client.get('/product/9300657007553')
        all_products = Products.query.all()
        this_product = Products.query.get('9300657007553')
        self.assertIn(this_product, all_products)
        self.assertEqual(res.status_code, 201)
        self.assertIn(b'Heinz', res.data)
