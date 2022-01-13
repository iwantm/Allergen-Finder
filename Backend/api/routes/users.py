from flask_jwt_extended.utils import get_current_user
from api import app, db, api
from flask import request
from flask_restx import Resource
from api.models import Users
import datetime
from flask_jwt_extended import create_access_token, get_jwt_identity, jwt_required, create_refresh_token


@api.route('/auth/register')
class Register(Resource):
    def post(self):
        body = request.get_json()
        user = Users(**body)
        user.hash_password()
        db.session.add(user)
        db.session.commit()
        id = user.id
        access_token = create_access_token(identity=str(user.id))
        refresh_token = create_refresh_token(identity=str(user.id))
        return {'id': str(id), "access_token": access_token, "refresh_token": refresh_token}, 201


@api.route('/auth/login')
class Login(Resource):
    def post(self):
        body = request.get_json()
        user = Users.query.filter_by(email=body.get('email')).first()
        authorized = user.check_password(body.get('password'))
        if not authorized:
            return {'error': 'Email or password invalid'}, 401

        access_token = create_access_token(identity=str(user.id))
        refresh_token = create_refresh_token(identity=str(user.id))
        return {'access_token': access_token, "refresh_token": refresh_token}, 200


@api.route('/auth/refresh')
class Refresh(Resource):
    @jwt_required(refresh=True)
    def post(self):
        identity = get_jwt_identity()
        access_token = create_access_token(identity=identity)
        return {"access_token": access_token}

# @api.route('/auth/user')
# class GetUser(Resource):
#     @jwt_required()
#     def get(self):
#         user_id = get_jwt_identity()
#         user_name = Users.query.filter_by(id=user_id).first().user_name
#         return {"access_token": user_name}
