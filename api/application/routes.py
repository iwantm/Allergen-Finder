
from application import app, db, api
from flask import render_template, redirect, url_for, request, jsonify
from flask_restx import Resource
from application.models import Products
from functions.searchApi import search_product
from functions.addToDatabase import add_to_db
import simplejson


@api.route('/product/<string:barcode>')
class Product(Resource):
    def get(self, barcode):
        product = Products.query.get(barcode)
        if not product:
            product = search_product(barcode)
            add_to_db(db, product)
            return product, 201
        else:
            product = product.as_dict()
            return product
