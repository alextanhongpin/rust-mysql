#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();
    match connection.execute("SELECT USER()") {
        Ok(val) => println!("value is {}", val),
        Err(err) => println!("got error {}", err),
    }
    println!("Hello, world!");
}
