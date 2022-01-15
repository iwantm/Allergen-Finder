from tests.test_base import TestBase
from api import app, db
from api.models import Users
from unittest.mock import patch
import json

class TestApplication(TestBase):
    def test_register_user(self):
        data = {
            "user_name": "iwantm",
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data)
        user = Users.query.filter_by(user_name="iwantm").first()
        users = Users.query.all()
        self.assertIn(user,users)
        self.assertIn(b'access_token', res.data)
        self.assertIn(b'refresh_token', res.data)
        self.assertIn(b'\"id\": \"2\"',res.data)
        self.assertEqual(res.status_code, 201)

    def test_register_user_already_exists(self):
        data = {
            "user_name": "test",
            "email": "test@test.test",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data)
        self.assertIn(b'user already exists',res.data)
        self.assertEqual(res.status_code, 406)

    def test_login(self):
        data1 = {
            "user_name": "iwantm",
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data1)
        data2 = {
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/login", json=data2)
        self.assertIn(b'access_token', res.data)
        self.assertIn(b'refresh_token', res.data)
        self.assertEqual(res.status_code, 200)

    def test_login_password_incorrect(self):
        data1 = {
            "user_name": "iwantm",
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data1)
        data2 = {
            "email": "test@iwantm.me",
            "password": "test12345"
        }
        res = self.client.post("/auth/login", json=data2)
        self.assertIn(b'Email or password invalid', res.data)
        self.assertEqual(res.status_code, 401)

    def test_login_email_incorrect(self):
        data1 = {
            "user_name": "iwantm",
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data1)
        data2 = {
            "email": "test123@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/login", json=data2)
        self.assertIn(b'Email or password invalid', res.data)
        self.assertEqual(res.status_code, 401)

    def test_refresh(self):
        data1 = {
            "user_name": "iwantm",
            "email": "test@iwantm.me",
            "password": "test123"
        }
        res = self.client.post("/auth/register",json=data1)
        refresh_token = json.loads(res.data)["refresh_token"]
        headers = {
        'Authorization': 'Bearer {}'.format(refresh_token)}
        res2 = self.client.post("/auth/refresh" ,headers=headers)
        self.assertEqual(res2.status_code, 200)
        self.assertIn(b'access_token', res2.data)