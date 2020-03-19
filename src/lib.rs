#[macro_use]
extern crate diesel;
extern crate config as rs_config;
extern crate dotenv;
extern crate log;
extern crate bigdecimal;

#[macro_use]
extern crate juniper;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod config;
pub mod ctx;
pub mod database;
pub mod graphql;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
