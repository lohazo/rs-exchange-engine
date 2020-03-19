extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::exchange_exchangeorder::dsl::*;

    let connection = establish_connection();
    let results = exchange_exchangeorder.filter(is_market_order.eq(false))
        .limit(5)
        .load::<Order>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.status);
        println!("----------\n");
        println!("{}", post.is_market_order);
    }
}