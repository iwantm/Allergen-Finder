// @generated automatically by Diesel CLI.

diesel::table! {
    products (barcode) {
        barcode -> Text,
        product_name -> Nullable<Text>,
        ingredients -> Array<Nullable<Text>>,
        allergens -> Nullable<Array<Nullable<Text>>>,
        traces -> Nullable<Array<Nullable<Text>>>,
        user_id -> Nullable<Text>,
    }
}
