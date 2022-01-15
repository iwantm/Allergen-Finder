import unittest
from flask import Flask
from api import app, db
from api.models import Products, Users
from flask_testing import TestCase
import os


class TestBase(TestCase):
    def create_app(self):
        app.config.update(
            SQLALCHEMY_DATABASE_URI=os.getenv('TEST_DATABASE_URL'),
            SECRET_KEY="TEST_SECRET_KEY",
            DEBUG=True,
            TESTING=True
        )
        return app

    def setUp(self):
        db.drop_all()
        db.create_all()
        new_product1 = Products(barcode='1',
                                productName='testProd',
                                ingredients=['fun', 'things'],
                                allergens='oofed',
                                allergensTags=['fun', 'things'],
                                traces=['fun', 'things'])
        new_product2 = Products(barcode='2',
                                productName='testProd',
                                ingredients=['fun', 'things'],
                                allergens='oofed',
                                allergensTags=['fun', 'things'],
                                traces=['fun', 'things'])
        new_user1 = Users(user_name='test',
                          email='test@test.test',
                          password='test123'
                          )
        db.session.add(new_product1)
        db.session.add(new_product2)
        db.session.add(new_user1)
        db.session.commit()

    def tearDown(self):
        db.session.remove()
        db.drop_all()
