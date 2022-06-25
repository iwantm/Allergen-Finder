from flask import Flask
from flask_restx import Api
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt
import os
from flask_jwt_extended import JWTManager
from datetime import timedelta

app = Flask(__name__)
app.config.update(
    SECRET_KEY=os.getenv('SECRET_KEY'),
    SQLALCHEMY_DATABASE_URI=os.getenv('DATABASE_URL'),
    SQLALCHEMY_TRACK_MODIFICATIONS=False,
    JWT_SECRET_KEY=os.getenv('JWT_SECRET_KEY'),
    JWT_ACCESS_TOKEN_EXPIRES= timedelta(hours=1),
    JWT_REFRESH_TOKEN_EXPIRES= timedelta(days=30)
)


db = SQLAlchemy(app)
api = Api(app)
bcrypt = Bcrypt(app)
jwt = JWTManager(app)

from api.routes import users, products
