
from api import app, db, api
from flask_restx import Resource
from api.models import Products, Users
from flask import request
from functions.productFunctions import search_product, add_to_db
from flask_jwt_extended import jwt_required, get_jwt_identity
import json


@api.route('/product/<string:barcode>')
class Product(Resource):
    @jwt_required()
    def get(self, barcode):
        product = Products.query.get(barcode)
        if not product:
            product = search_product(barcode)
            add_to_db(db, product)
            product = Products.query.get(barcode).as_dict()
            return product, 201
        else:
            product = product.as_dict()
            return product


@api.route('/product/create')
class CreateProduct(Resource):
    @jwt_required()
    def post(self):
        body = request.get_json()
        product = Products.query.get(body.get("barcode"))
        if product:
            product = product.as_dict()
            product["msg"] = "Product Already Exists"
            return product, 303
        if not product:
            user_id = get_jwt_identity()
            user_name = Users.query.filter_by(id=user_id).first().user_name
            product = {
                "barcode": body.get("barcode"),
                "productName": body.get("productName"),
                "ingredients": body.get("ingredients"),
                "allergens": body.get("allergens"),
                "allergensTags": body.get("allergensTags"),
                "traces": body.get("traces")}
            add_to_db(db, product, user_id, user_name)
            return {"msg": "product created", "user": user_name, "user_id": user_id}, 201


@api.route('/product/like')
class Like(Resource):
    @jwt_required()
    def post(self):
        body = request.get_json()
        product = Products.query.get(body.get("barcode"))

        if product:
            product.likes = product.likes + 1
            db.session.commit()
            return {"product": product.productName, "likes": product.likes}, 200
        else:
            return {"err": "Product does not exist"}, 404


@api.route('/product/dislike')
class Like(Resource):
    @jwt_required()
    def post(self):
        body = request.get_json()
        product = Products.query.get(body.get("barcode"))

        if product:
            product.likes = product.likes - 1
            db.session.commit()
            return {"product": product.productName, "likes": product.likes}, 200
        else:
            return {"err": "Product does not exist"}, 404
