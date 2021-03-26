import openfoodfacts


def search_product(barcode):
    search_result = openfoodfacts.products.get_product(barcode)
    product = {
        "barcode": search_result['code'],
        "productName": search_result['product']['product_name_en'],
        "ingredients": search_result['product']['ingredients_tags'],
        "allergens": search_result['product']['allergens'],
        "allergensTags": search_result['product']['allergens_tags'],
        "traces": search_result['product']['traces_tags']}
    return product
