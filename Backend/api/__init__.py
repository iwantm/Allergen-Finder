from flask import Flask
from flask_restx import Api
from flask_sqlalchemy import SQLAlchemy
import os

app = Flask(__name__)
app.config.update(
    DEBUG=True,
    SECRET_KEY='acvs',
    SQLALCHEMY_DATABASE_URI=os.getenv('DATABASE_URL'),
    SQLALCHEMY_TRACK_MODIFICATIONS=False,
    JWT_SECRET_KEY='t1NP63m4wnBg6nyHYKfmc2TpCOGI4nss'
)


db = SQLAlchemy(app)
api = Api(app)

from api import routes
