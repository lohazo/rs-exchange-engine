use juniper::{FieldResult};
use log::{info, warn};
use diesel::prelude::*;

use crate::ctx::Ctx;
use crate::models::*;
use crate::*;

pub struct Query;

#[juniper::object(
  Context = Ctx
)]
impl Query {
  fn hello() -> FieldResult<String> {
    Ok(String::from("world"))
  }

  fn all_orders(ctx: &Ctx, limit: i32) -> FieldResult<Vec<Order>> {
    let conn = &*ctx.db_connect();
    let connection = establish_connection();
    use crate::schema::exchange_exchangeorder::dsl::*;

    let results = exchange_exchangeorder.filter(is_market_order.eq(false))
        .limit(limit as i64)
        // .order(exchange_exchangeorder::created_at.desc())
        .load::<Order>(&connection)
        .expect("Error loading posts");

    info!("cac==========");
    Ok(results)
  }
}

pub struct Mutation;

#[juniper::object(
  Context = Ctx
)]
impl Mutation {
  fn muu() -> FieldResult<String> {
    Ok(String::from("world"))
  }
}

/*************************
 * Create schema
 **************************/
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn build_schema() -> Schema {
  Schema::new(Query, Mutation)
}