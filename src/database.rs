// use ctx::Ctx;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
// use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::collections::HashMap;
// use std::thread;

/// Connect to our Postgres database
///
/// We use a "connection pool" that can be shared by
/// the different threads instead of a single connection or open
/// a new connection systematically.
pub fn db_pool(
  config: &HashMap<String, String>,
) -> r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::PgConnection>> {
  let dabatabase_uri = config.get("database_uri").unwrap().as_str();
  let manager = ConnectionManager::<PgConnection>::new(dabatabase_uri);

  println!(
    "🔍  Attempt to connect to database \"{}\" in progress ...",
    &dabatabase_uri
  );

  // let manager = PostgresConnectionManager::new(dabatabase_uri, TlsMode::None).unwrap();
  // let pool = r2d2::Pool::new(manager).expect("Can not connect to the database!");
  let pool = r2d2::Pool::new(manager).expect("Faled to create pool.");

  println!("🎉  Connection to the database successful!");

  pool
}
