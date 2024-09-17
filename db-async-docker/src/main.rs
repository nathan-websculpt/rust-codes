// docker compose up -d db                  // <__ means: runs detached db service
// docker ps -a                             // <__ means: list all containers
// docker exec -it db psql -U postgres      // <__ means: enter into db container ... exec, interactive on db: run psql cmd with user postgres
//
//
// from new terminal, run (to see errors in compiler): 
// docker compose build
//
//
// run this application
// docker compose up rustapp
// 
// TEST: From first terminal (which should be in postgres=#)
// \dt
// ^^^ should show tables w/ a _sqlx_migrations table
// select * from public.person;
// ^^^ see data

#![allow(dead_code)]
use sqlx::FromRow;
use std::error::Error;

#[derive(Debug, FromRow)]
struct Person {
    pub full_name: String,
}

async fn create_person(person: &Person, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO person (full_name) VALUES ($1)";
    sqlx::query(query)
        .bind(&person.full_name)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read_person(conn: &sqlx::PgPool) -> Result<Vec<Person>, Box<dyn Error>> {
    let q = "SELECT full_name FROM person";
    let query = sqlx::query_as::<_, Person>(q);

    let people = query.fetch_all(conn).await?;

    Ok(people)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_conn = env!("DATABASE_URL");

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
