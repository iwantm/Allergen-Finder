
from application import app, db, api
from flask import render_template, redirect, url_for, request
from flask_restx import Resource
from application.models import Products
from functions.searchApi import search_product


@api.route('/<string:barcode>')
class Product(Resource):
    def get(self, barcode):
        return search_product(barcode)
