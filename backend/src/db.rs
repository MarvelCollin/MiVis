use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to the database");

    Ok(pool)
}