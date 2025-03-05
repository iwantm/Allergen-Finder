use rocket::{get, post, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    functions::product::get_product,
    models::{auth::AuthenticatedUser, error::ApiError, product::Product},
    DbConn,
};

#[get("/product/<code>")]
pub async fn get_product_endpoint(
    code: &str,
    mut db_pool: Connection<DbConn>,
    _user: AuthenticatedUser,
) -> Result<Json<Product>, ApiError<String>> {
    Ok(Json(get_product(code, &mut **db_pool).await?))
}

#[post("/product", data = "<product>")]
pub async fn post_product_endpoint(
    product: Json<Product>,
    mut db_pool: Connection<DbConn>,
    user: AuthenticatedUser,
) -> Result<Json<Product>, ApiError<String>> {
    let mut new_product = product.into_inner();
    new_product.user_id = Some(user.user_id);

    let inserted_product = Product::insert(&mut **db_pool, &new_product).await?;
    Ok(Json(inserted_product))
}
