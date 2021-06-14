
from api import app, db, api
from flask import render_template, redirect, url_for, request, jsonify
from flask_restx import Resource
from api.models import Products, Users
from functions.productFunctions import search_product, add_to_db
import datetime
from flask_jwt_extended import create_access_token, jwt_required


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


@api.route('/auth/register')
class SignupApi(Resource):
    def post(self):
        body = request.get_json()
        user = Users(**body)
        user.hash_password()
        db.session.add(user)
        db.session.commit()
        id = user.id
        return {'id': str(id)}, 200


@api.route('/auth/login')
class LoginApi(Resource):
    def post(self):
        body = request.get_json()
        user = Users.query.filter_by(email=body.get('email')).first()
        authorized = user.check_password(body.get('password'))
        if not authorized:
            return {'error': 'Email or password invalid'}, 401

        expires = datetime.timedelta(days=7)
        access_token = create_access_token(
            identity=str(user.id), expires_delta=expires)
        return {'token': access_token}, 200
