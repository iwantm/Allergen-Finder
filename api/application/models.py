from application import db


class Products(db.Model):
    barcode = db.Column(db.Integer, primary_key=True)
    productName = db.Column(db.String(255))
    ingredients = db.Column(db.String(255))
    allergens = db.Column(db.String(255))
    allergensTags = db.Column(db.String(255))
    traces = db.Column(db.String(255))
