use serde::Deserialize;

use super::product::Product;

#[derive(Deserialize, Debug)]
pub struct ResultInfo {
    pub id: String,
    pub lc_name: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProductData {
    pub code: String,
    pub product: Option<Product>,
    pub result: ResultInfo,
    pub status: String,
}
