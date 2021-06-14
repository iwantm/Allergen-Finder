from api import db


class Products(db.Model):
    barcode = db.Column(db.String(255), primary_key=True)
    productName = db.Column(db.String(255))
    ingredients = db.Column(db.ARRAY(db.String(256)))
    allergens = db.Column(db.String(255))
    allergensTags = db.Column(db.ARRAY(db.String(256)))
    traces = db.Column(db.ARRAY(db.String(256)))
    created_by = db.Column(
        db.Integer, db.ForeignKey('users.id'), nullable=True)

    def as_dict(self):
        return {c.name: getattr(self, c.name) for c in self.__table__.columns}
