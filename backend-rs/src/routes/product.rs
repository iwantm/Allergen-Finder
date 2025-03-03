use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    functions::product::get_product,
    models::{api::ApiError, product::Product},
    DbConn,
};

#[get("/product/<code>")]
pub async fn get_product_endpoint(
    code: &str,
    mut db_pool: Connection<DbConn>,
) -> Result<Json<Product>, ApiError<String>> {
    Ok(Json(get_product(code, &mut **db_pool).await?))
}
