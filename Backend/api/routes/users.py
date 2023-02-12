
from api import app, db, api
from flask import request
from flask_restx import Resource
from api.models import Users
from flask_jwt_extended import create_access_token, get_jwt_identity, jwt_required, create_refresh_token


@api.route('/auth/register')
class Register(Resource):
    def post(self):
        body = request.get_json()
        user = Users.query.filter_by(user_name=body.get('user_name')).first()
        email = Users.query.filter_by(email=body.get('email')).first()
        if not user and not email:
            user = Users(**body)
            user.hash_password()
            db.session.add(user)
            db.session.commit()
            id = user.id
            access_token = create_access_token(identity=str(user.id))
            refresh_token = create_refresh_token(identity=str(user.id))
            return {"success": True, "message": "user created", "data": {'access_token': access_token, "refresh_token": refresh_token}}, 201

        return {"success": False, "message": "user already exists", "data": {}}, 406


@api.route('/auth/login')
class Login(Resource):
    def post(self):
        body = request.get_json()
        user = Users.query.filter_by(email=body.get('email')).first()
        if not user:
            return {"success": False, "message": "Email or password invalid'", "data": {}}, 401
        authorized = user.check_password(body.get('password'))
        if not authorized:
            return {"success": False, "message": "Email or password invalid'", "data": {}}, 401

        access_token = create_access_token(identity=str(user.id))
        refresh_token = create_refresh_token(identity=str(user.id))

        return {"success": True, "message": "user logged in", "data": {'access_token': access_token, "refresh_token": refresh_token}}, 200


@api.route('/auth/refresh')
class Refresh(Resource):
    @jwt_required(refresh=True)
    def post(self):
        identity = get_jwt_identity()
        access_token = create_access_token(identity=identity)
        return {"success": True, "message": "token refreshed", "data": {"access_token": access_token}}, 200


@api.route('/auth/user')
class GetUser(Resource):
    @jwt_required()
    def get(self):
        user_id = get_jwt_identity()
        user = Users.query.filter_by(id=user_id).first()
        return {"success": True, "message": "user info", "data": {"user_name": user.user_name, "email": user.email}}, 200
