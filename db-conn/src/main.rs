extern crate postgres;
extern crate dotenv;

use dotenv::dotenv;
use postgres::{Client, NoTls};

fn main() {
    dotenv().ok();

    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    // Execute the function: tst_db_function
    let result = client.query("SELECT tst_db_function($1)", &[&"hello".to_string()]).unwrap();
    let row = result.get(0);
    let value: String = row.get(0);

    println!("Result: {}", value);
}
