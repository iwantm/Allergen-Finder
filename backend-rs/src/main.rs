pub mod functions;
pub mod models;
pub mod routes;
pub mod schema;

use rocket::launch;
use rocket_db_pools::{diesel, Database};

#[derive(Database)]
#[database("postgres_db")]
pub struct DbConn(diesel::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![routes::product::get_product_endpoint])
        .attach(DbConn::init())
}
