use sqlx::Pool;
use state::InitCell;

pub static POSTGRES_POOL: InitCell<Pool<sqlx::Postgres>> = InitCell::new();

#[derive(Debug)]
pub struct CnctdDB;

impl CnctdDB {
    pub async fn new_pool(url: &str) -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            ?;

        Ok(pool)
    }
    
    pub async fn start(url: &str) -> anyhow::Result<()> {
        let pool = Self::new_pool(url).await?;
        POSTGRES_POOL.set(pool);

        Ok(())
    }

    pub fn get_pool() -> anyhow::Result<&'static Pool<sqlx::Postgres>> {
        Ok(POSTGRES_POOL.get())
    }

    
}