use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;
use juniper::{LookAheadSelection};

#[derive(juniper::GraphQLObject)]
#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub username: Option<String>,
  pub password: Option<String>
}

// The order of the field must match the order in schema.rs
#[derive(Queryable)]
pub struct Order {
  pub id: i32,
  pub date_added: DateTime<Utc>,
  pub date_modified: DateTime<Utc>,
  pub deleted_at: Option<DateTime<Utc>>,
  pub is_deleted: bool,
  pub status: String,
  pub r#type: String,
  pub price: BigDecimal,
  pub fee_rate: BigDecimal,
  pub amount: BigDecimal,
  pub amount_left: BigDecimal,
  pub currency_pair_id: i32,
  pub user_id: i32,
  pub cht_price: BigDecimal,
  pub using_cht_fee: bool,
  pub actual_rate: BigDecimal,
  pub is_market_order: bool,
  pub market_price: Option<BigDecimal>,
  pub stop_price: Option<BigDecimal>,
}

// impl User {
//     fn from_lookahead(selection: LookAheadSelection) -> Self {
//         // do something with the selection here
//     }
// }

#[juniper::object]
impl Order {
  fn id(&self) -> i32 {
    self.id
  }

  fn date_added(&self) -> DateTime<Utc> {
    self.date_added
  }

  fn status(&self) -> &str {
    &self.status
  }

  #[graphql(name = "type")]
  fn order_type(&self) -> &str {
    &self.r#type
  }

  fn price(&self) -> String {
    self.price.to_string()
  }

  fn fee_rate(&self) -> String {
    self.fee_rate.to_string()
  }

  fn amount(&self) -> String {
    self.amount.to_string()
  }

  fn amount_left(&self) -> String {
    self.amount_left.to_string()
  }

  fn cht_price(&self) -> String {
    self.cht_price.to_string()
  }

  fn actual_rate(&self) -> String {
    self.actual_rate.to_string()
  }

  fn currency_pair_id(&self) -> i32 {
    self.currency_pair_id
  }

  fn user_id(&self) -> i32 {
    self.user_id
  }

  fn using_cht_fee(&self) -> bool {
    self.using_cht_fee
  }

  fn is_market_order(&self) -> bool {
    self.is_market_order
  }

  fn market_price(&self) -> Option<String> {
    match &self.market_price {
      Some(v) => Some(v.to_string()),
      None => None,
    }
  }

  fn stop_price(&self) -> Option<String> {
    match &self.stop_price {
      Some(v) => Some(v.to_string()),
      None => None,
    }
  }

  // fn user(&self, &executor) -> User {
  //   let lookahead = executor.look_ahead();
  //   User::from_lookahead(lookahead)
  // } 
}
