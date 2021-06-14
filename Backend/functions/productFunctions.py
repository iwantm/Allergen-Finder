import openfoodfacts
from api.models import Products


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


def search_product(barcode):
    search_result = openfoodfacts.products.get_product(barcode)
    try:
        productName = search_result['product']['product_name_en']
    except:
        productName = search_result['product']['product_name']

    product = {
        "barcode": search_result['code'],
        "productName": productName,
        "ingredients": search_result['product']['ingredients_tags'],
        "allergens": search_result['product']['allergens'],
        "allergensTags": search_result['product']['allergens_tags'],
        "traces": search_result['product']['traces_tags']}
    return product
