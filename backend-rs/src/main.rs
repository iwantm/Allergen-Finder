pub mod functions;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

use crate::models::auth::Auth0Config;
use crate::models::auth::Jwks;
use reqwest::Client;
use rocket::launch;
use rocket_db_pools::{diesel, Database};
use std::sync::Arc;
use utils::catchers;

#[derive(Database)]
#[database("my_db")]
pub struct DbConn(diesel::PgPool);

#[launch]
fn rocket() -> _ {
    let http_client = Client::new();

    let auth0_config = Auth0Config {
        domain: "dev-ftrjhtyxl1c6zv73.uk.auth0.com".to_string(),
        audience: "allergen-finder".to_string(),
    };

    let jwks_cache = Arc::new(Jwks::new(http_client));

    rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::product::get_product_endpoint,
                routes::product::post_product_endpoint
            ],
        )
        .register("/", catchers::catchers())
        .manage(auth0_config)
        .manage(jwks_cache)
        .attach(DbConn::init())
}
