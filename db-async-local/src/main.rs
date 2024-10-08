#![allow(dead_code)]
use dotenv::dotenv;
use sqlx::FromRow;
use sqlx::Row;
use std::error::Error;

// note: The compiler only re-runs proc macros when one or more source files have changed
// so changes to migrations will not always be picked up
//
// install sqlx-cli:
// cargo install sqlx-cli
//
//
// then
// run in command line on the project root
// ~/.cargo/bin/sqlx migrate build-script
//
// this will generate a file called `build.rs` in the project root
// this will trigger recompilation when a new migration is added
// 
// 
// There are four options for fetching data: 
// fetch_one (returns a single row)[assumes row exists, returns error if it doesn't], 
// fetch_optional (instead of an error it will return None), 
// fetch_all (returns all rows as a Vec),
// and fetch (returns all as a stream lifetime that can be itertated over)[more async than fetch_all]
//
//
// Example ... iterating a fetch_all:
// let people = rows.iter().map(|row| Person { full_name: row.get("full_name") }).collect();
// 
// 
// Example ... iterating a fetch:
// let mut people = vec![];
// while let Some(row) = rows.try_next().await? {
//     people.push(Person { full_name: row.get("full_name") 
// });
//
//
// Perform (atomic) operations together (or, not at all) with Transactions
// start a transaction:
// let mut txn = conn.begin().await?;
// ^^^ returns Transaction Type
// You then use this as the Type for queries:
// like this ... .fetch_one(&mut txn) ..... .execute(&mut txn)
// then: txn.commit().await?
//
    
#[derive(Debug, FromRow)]
struct Person {
    pub full_name: String,
}

async fn create_person(person: &Person, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO person (full_name) VALUES ($1)";
    sqlx::query(query).bind(&person.full_name).execute(pool).await?;
    Ok(())
}

async fn read_person(conn: &sqlx::PgPool) -> Result<Vec<Person>, Box<dyn Error>> {
    // let q = "SELECT id, full_name, created_at FROM person";
    let q = "SELECT full_name FROM person";
    let query = sqlx::query_as::<_, Person>(q);

    let people = query.fetch_all(conn).await?;

    Ok(people)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // loads the environment variables
    let db_conn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = sqlx::postgres::PgPool::connect(&db_conn).await?;

    //quick test
    // let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;
    // let sum: i32 = res.get("sum");
    // println!("sum: {}", sum);

    sqlx::migrate!("./migrations").run(&pool).await?;

    let person_one = Person {
        full_name: "Jerry Seinfeld".to_string(),
    };

    let person_two = Person {
        full_name: "George Castanza".to_string(),
    };

    create_person(&person_one, &pool).await?;
    create_person(&person_two, &pool).await?;

    let persons = read_person(&pool).await?;
    println!("people: {:?}", persons);

    Ok(())
}
