use chrono::NaiveDate;
use sqlx::{Executor, Row};

use cherry::Cherry;
use cherry::query::{QueryExecutor, SetColumn};
use cherry::sqlite::SqlitePool;
use cherry::sqlx::types::Json;
use cherry_derive::Cherry;

async fn init() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(include_str!("migrations.sql")).await.unwrap();
    pool
}

#[async_std::test]
async fn test_insert_one() {
    let pool = init().await;
    let user = User { id: 100, name: "test_insert_one".into(), age: 25, };
    // let r = SqliteQuery::new_insert(&user)
    //     .set_column("").execute(&pool).await.unwrap();
    // let r = Query::new_insert(&user).execute(&pool).await.unwrap();
    let r = user.insert().execute(&pool).await.unwrap();
    assert_eq!(1, r.rows_affected());

    let sql = "select * from user where id = 100";
    let row = sqlx::query(sql).fetch_one(&pool).await.unwrap();
    let result = User { id: row.get("id"), name: row.get("name"), age: row.get("age"), };
    assert_eq!(&user, &result);
}

#[async_std::test]
async fn test_insert_ignore() {

}

#[async_std::test]
async fn test_insert_update() {

}

#[async_std::test]
async fn test_insert_replace() {

}


#[derive(Debug, Cherry, Eq, PartialEq)]
#[cherry(database = "sqlite")]
struct User {
    id: u32,
    name: String,
    age: u8,
}

#[derive(Debug, Cherry, Eq, PartialEq)]
#[cherry(database = "sqlite")]
struct Book {
    id: u32,
    name: String,
    authors: Json<Vec<String>>,
    edition: u8,
    published_date: NaiveDate,
}

#[test]
fn check_type() {

}
