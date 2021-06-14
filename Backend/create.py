from api import db
from api.models import Products, Users
db.drop_all()
db.create_all()
