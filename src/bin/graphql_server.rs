#![feature(decl_macro, proc_macro_hygiene)]

use rocket::{response::content, State};

use diesel_demo::config;
use diesel_demo::ctx::Ctx;
use diesel_demo::database;
use diesel_demo::graphql;
use diesel_demo::graphql::Schema;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
  juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
  context: State<Ctx>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
  context: State<Ctx>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}

fn main() {
  let config = config::get();
  let ctx = Ctx {
    db_pool: database::db_pool(&config),
    config,
  };

  rocket::ignite()
    .manage(ctx)
    .manage(graphql::build_schema())
    .mount(
      "/",
      rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
    )
    .launch();
}
