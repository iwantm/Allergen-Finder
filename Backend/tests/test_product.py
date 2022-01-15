from tests.test_base import TestBase
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

    @patch('api.routes.products.get_jwt_identity')
    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_create_product(self, mock_jwt_required, mock_jwt_identity):
        mock_jwt_identity.return_value = '1'
        data = {
            "barcode": "123",
            "productName": "Testing",
            "ingredients": [
                "en:test",
                "en:test2"
            ],
            "allergens": "en:test,en:test2",
            "allergensTags": [
                "en:test",
                "en:test2"
            ],
            "traces": []
        }
        res = self.client.post("/product/create", json=data)
        new_product = Products.query.get('123')
        all_products = Products.query.all()
        self.assertEqual(res.status_code, 201)
        self.assertIn(new_product, all_products)
        self.assertIn(b'created', res.data)

    @patch('api.routes.products.get_jwt_identity')
    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_create_duplicate_product(self, mock_jwt_required, mock_jwt_identity):
        mock_jwt_identity.return_value = '1'
        data = {
            "barcode": "1",
            "productName": "Testing",
            "ingredients": [
                "en:test",
                "en:test2"
            ],
            "allergens": "en:test,en:test2",
            "allergensTags": [
                "en:test",
                "en:test2"
            ],
            "traces": []
        }
        res = self.client.post("/product/create", json=data)
        self.assertEqual(res.status_code, 303)
        self.assertIn(b'Already Exists', res.data)
        self.assertIn(b'testProd', res.data)

    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_liking_product(self, mock_jwt_required):
        data = {"barcode": "1"}
        res = self.client.post("/product/like", json=data)
        like_count = Products.query.get("1").likes
        self.assertEqual(res.status_code, 200)
        self.assertIn(b'1', res.data)
        self.assertIn(b'testProd', res.data)
        self.assertEqual(like_count, 1)
        res2 = self.client.post("/product/like", json=data)
        like_count = Products.query.get("1").likes
        self.assertIn(b'2', res2.data)
        self.assertIn(b'testProd', res2.data)
        self.assertEqual(like_count, 2)

    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_liking_product_not_exist(self, mock_jwt_required):
        data = {"barcode": "123"}
        res = self.client.post("/product/like", json=data)
        self.assertEqual(res.status_code, 404)
        self.assertIn(b'Product does not exist', res.data)

    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_disliking_product(self, mock_jwt_required):
        data = {"barcode": "2"}
        res = self.client.post("/product/dislike", json=data)
        like_count = Products.query.get("2").likes
        self.assertEqual(res.status_code, 200)
        self.assertIn(b'-1', res.data)
        self.assertIn(b'testProd', res.data)
        self.assertEqual(like_count, -1)
        res2 = self.client.post("/product/dislike", json=data)
        like_count = Products.query.get("2").likes
        self.assertIn(b'-2', res2.data)
        self.assertIn(b'testProd', res2.data)
        self.assertEqual(like_count, -2)

    @patch('flask_jwt_extended.view_decorators.verify_jwt_in_request')
    def test_dosliking_product_not_exist(self, mock_jwt_required):
        data = {"barcode": "123"}
        res = self.client.post("/product/dislike", json=data)
        self.assertEqual(res.status_code, 404)
        self.assertIn(b'Product does not exist', res.data)
