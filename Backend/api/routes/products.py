
from api import app, db, api
from flask_restx import Resource
from api.models import Products
from functions.productFunctions import search_product, add_to_db
from flask_jwt_extended import jwt_required


@api.route('/product/<string:barcode>')
class Product(Resource):
    @jwt_required()
    def get(self, barcode):
        product = Products.query.get(barcode)
        if not product:
            product = search_product(barcode)
            add_to_db(db, product)
            return product, 201
        else:
            product = product.as_dict()
            return product
