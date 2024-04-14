use sea_orm::{Database, DatabaseConnection};
// use sqlx::{postgres::PgQueryResult, query, Pool, Postgres};
use state::InitCell;
// pub use cnctd_db_macros::*;
pub use sea_orm;

pub static CONNECTION: InitCell<DatabaseConnection> = InitCell::new();

pub trait SqlInsertable {
    fn insert_query(&self) -> String;
}

#[derive(Debug)]
pub struct CnctdDB;

impl CnctdDB {
    pub async fn connect(url: &str) -> anyhow::Result<DatabaseConnection> {
        Ok(Database::connect(url).await?)
    }

    pub async fn start(url: &str) -> anyhow::Result<()> {
        let conn = Self::connect(url).await?;
        CONNECTION.set(conn);

        Ok(())
    }

    // pub async fn new_pool(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    //     let pool = sqlx::postgres::PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect(url)
    //         .await
    //         ?;

    //     Ok(pool)
    // }
    
    // pub async fn start(url: &str) -> anyhow::Result<()> {
    //     let pool = Self::new_pool(url).await?;
    //     POSTGRES_POOL.set(pool);

    //     Ok(())
    // }

    // pub fn get_pool() -> anyhow::Result<&'static Pool<Postgres>> {
    //     Ok(POSTGRES_POOL.get())
    // }

    // pub async fn execute(query: &str) -> anyhow::Result<PgQueryResult> {
    //     let pool = Self::get_pool()?;
    //     let result = sqlx::query(query).execute(pool).await?;
        
    //     Ok(result)
    // }

    // pub async fn insert<T>(table: T) -> anyhow::Result<PgQueryResult>
    // where T: SqlInsertable {
    //     let query = table.insert_query();
    //     println!("Query: {}", query);

    //     let result = Self::execute(&query).await?;

    //     Ok(result)
    // }
}