from application import db
from application.models import Products
db.drop_all()
db.create_all()
