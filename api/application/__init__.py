from flask import Flask
from flask_sqlalchemy import SQLAlchemy
import os
from flask_restx import Api

basedir = os.path.abspath(os.path.dirname(__file__))
# postgres_local_base = os.environ['DATABASE_URL']
app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgres:///' + \
    os.path.join(basedir, 'flask_boilerplate_main.db')
app.config['SECRET_KEY'] = os.getenv['SECRET_KEY']
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False

db = SQLAlchemy(app)
api = Api(app)

from application import routes
