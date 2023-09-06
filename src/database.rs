use crate::config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = Config::new()
        .map
        .get("DB_URL")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}
