// @generated automatically by Diesel CLI.

diesel::table! {
    products (barcode) {
        barcode -> Text,
        product_name -> Nullable<Text>,
        ingredients -> Array<Nullable<Text>>,
        allergens -> Nullable<Array<Nullable<Text>>>,
        traces -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        oauth_provider -> Text,
        oauth_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
