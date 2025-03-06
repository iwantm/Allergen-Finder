use crate::schema::products;

use diesel::insert_into;

use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel::{prelude::RunQueryDsl, AsyncPgConnection};
use serde::{Deserialize, Serialize};

use super::error::{ApiError, ErrorStruct};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    #[serde(alias = "code")]
    pub barcode: String,
    pub product_name: Option<String>,
    #[serde(alias = "ingredients_tags")]
    pub ingredients: Vec<Option<String>>,
    #[serde(alias = "allergens_tags")]
    pub allergens: Option<Vec<Option<String>>>,
    #[serde(alias = "traces_tags")]
    pub traces: Option<Vec<Option<String>>>,
    #[serde(alias = "user_id")]
    pub user_id: Option<String>,
}

impl Product {
    pub async fn insert(
        db_pool: &mut AsyncPgConnection,
        other: &Self,
    ) -> Result<Self, ApiError<String>> {
        let call = insert_into(products::table)
            .values(other)
            .returning(Self::as_returning())
            .get_result(db_pool)
            .await;

        match call {
            Ok(product) => Ok(product),
            Err(e) => Err(ApiError::InternalServerError(ErrorStruct::new(
                "Failed to write from DB",
                Some(e.to_string()),
            ))),
        }
    }

    pub async fn get(
        db_pool: &mut AsyncPgConnection,
        query_barcode: &str,
    ) -> Result<Option<Self>, ApiError<String>> {
        let call = products::table
            .find(query_barcode)
            .get_result(db_pool)
            .await
            .optional();

        match call {
            Ok(product) => Ok(product),
            Err(e) => Err(ApiError::InternalServerError(ErrorStruct::new(
                "Failed to read from DB",
                Some(e.to_string()),
            ))),
        }
    }
}
