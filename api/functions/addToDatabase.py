from application.models import Products


def add_to_db(db, product):
    new_product = Products(
        barcode=product['barcode'],
        productName=product['productName'],
        ingredients=product['ingredients'],
        allergens=product["allergens"],
        allergensTags=product["allergensTags"],
        traces=product["traces"]
    )
    db.session.add(new_product)
    db.session.commit()
