use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use std::collections::HashMap;

// Type aliases to simplify a bit the types
type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PostgresPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// #[derive(Debug)]
pub struct Ctx {
  pub db_pool: PostgresPool,
  pub config: HashMap<String, String>,
}

impl Ctx {
  // to be able to recover a connection by simply doing "ctx.db_connect ()"
  pub fn db_connect(&self) -> PostgresPooledConnection {
    // self.db_pool.get().unwrap()
    self.db_pool.get().expect("Failed to get pooled connection")
  }
}

// allow our Ctx structure to be a valid context
// for Juniper (the GraphQL library).
// Our context will then be accessible from
// any resolver with "executor.context ()"
impl juniper::Context for Ctx {}
