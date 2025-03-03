use crate::models::api::ApiError;
use crate::models::off::ProductData;
use crate::models::product::Product;
use reqwest::get;
use rocket::serde::json::Json;
use rocket_db_pools::diesel::AsyncPgConnection;

async fn get_product_from_off(barcode: &str) -> Result<Product, ApiError<String>> {
    let url = format!("https://world.openfoodfacts.org/api/v3/product/{}.json&fields=code,product_name,allergens_tags,ingredients_tags,traces_tags", barcode);

    let resp = match get(url).await {
        Ok(resp) => resp,
        Err(_) => {
            return Err(ApiError::InternalServer(Json(
                "Failed to make request to Open Food Facts API".to_string(),
            )))
        }
    };

    let product_data = match resp.json::<ProductData>().await {
        Ok(product_data) => product_data,
        Err(_) => {
            return Err(ApiError::InternalServer(Json(
                "Failed to parse JSON response from Open Food Facts API".to_string(),
            )))
        }
    };

    match product_data.product {
        Some(product) => Ok(product),
        _none => Err(ApiError::NotFound(Json(
            format! {"Call to API Failed with error: {}",
            product_data.result.name},
        ))),
    }
}

pub async fn get_product(
    barcode: &str,
    db_pool: &mut AsyncPgConnection,
) -> Result<Product, ApiError<String>> {
    match Product::get(db_pool, &barcode).await {
        Ok(Some(product)) => Ok(product),
        Ok(_none) => Product::insert(db_pool, &get_product_from_off(barcode).await?).await,
        Err(e) => Err(e),
    }
}
