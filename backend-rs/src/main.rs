pub mod functions;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

use crate::models::auth::Auth0Config;
use crate::models::auth::Jwks;

use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use reqwest::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::launch;
use rocket::Rocket;
use rocket_db_pools::{diesel, Database};

use std::sync::Arc;
use utils::catchers;

#[derive(Database)]
#[database("my_db")]
pub struct DbConn(diesel::PgPool);

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct MigrateDb;
#[rocket::async_trait]
impl Fairing for MigrateDb {
    fn info(&self) -> Info {
        Info {
            name: "Diesel Migrations",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<rocket::Orbit>) {
        let pool = rocket.state::<DbConn>().expect("db connection");
        let mut conn = pool.get().await.expect("message");

        MIGRATIONS
            .run_pending_migrations(&mut *conn)
            .await
            .expect("Error running migrations");
    }
}

#[launch]
async fn rocket() -> _ {
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
        .attach(MigrateDb)
}
