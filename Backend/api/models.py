from api import db
from flask_bcrypt import generate_password_hash, check_password_hash


class Products(db.Model):
    barcode = db.Column(db.String(255), primary_key=True)
    productName = db.Column(db.String(255))
    ingredients = db.Column(db.ARRAY(db.String(256)))
    allergens = db.Column(db.String(255), nullable=True)
    allergensTags = db.Column(db.ARRAY(db.String(256)), nullable=True)
    traces = db.Column(db.ARRAY(db.String(256)), nullable=True)
    created_by = db.Column(
        db.Integer, db.ForeignKey('users.id'), nullable=True)
    created_by_name = db.Column(db.String(256), default='OpenFoodFacts')
    likes = db.Column(db.BigInteger, default=0)

    def as_dict(self):
        return {c.name: getattr(self, c.name) for c in self.__table__.columns}


class Users(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    user_name = db.Column(db.String(15), nullable=False, unique=True)
    email = db.Column(db.String(30), nullable=False, unique=True)
    password = db.Column(db.String(255), nullable=False)

    def hash_password(self):
        self.password = generate_password_hash(self.password).decode('utf8')

    def check_password(self, password):
        return check_password_hash(self.password, password)
