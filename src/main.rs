extern crate dotenv;
use cuid2::cuid;
use sqlx::{
    mysql::{MySqlPoolOptions, MySqlRow},
    Column, MySql, Pool, Row,
};
use tokio::time::Instant;

use dotenv::dotenv;
use std::env;

#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set 🤬"))
        .await?;

    // let init_result = init_schema(&pool).await;
    // if init_result.is_err() {
    //     panic!("Failed to init db schema {:?}", init_result);
    // }

    println!("Inserting images");

    let run_time = Instant::now();
    // for i in 0..1000 {
    let mut i = 0;
    loop {
        i += 1;
        let start_time = Instant::now();

        let result = query_poker_count(&pool).await;
        if result.is_err() {
            println!("Failed to query db {:?}", result);
        } else if let Ok(result) = result {
            // insertPoker(&pool).await.unwrap();
            println!(
                "Query no {} took {:#?} count: {:#?}",
                i,
                start_time.elapsed(),
                result.get::<i64, &str>("count(*)")
            );
        }
    }

    println!("total run time: {:#?}", run_time.elapsed());
    Ok(())
}

async fn query_poker_count(pool: &Pool<MySql>) -> Result<MySqlRow, sqlx::Error> {
    sqlx::query("select count(*) from (select vote.Poker.id from vote.Poker where 1 = 1) as sub")
        .fetch_one(pool)
        .await
}

async fn insertPoker(pool: &Pool<MySql>) -> Result<MySqlRow, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO vote.Poker (id, title, createdAt, updatedAt, createdByUserId, private) VALUES (? , 'title', '2021-01-01 00:00:00', '2021-01-01 00:00:00', ?, 1)",cuid().to_owned(), cuid().to_owned()
    )
    .fetch_one(pool)
    .await
}
