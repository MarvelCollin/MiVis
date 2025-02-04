use backend::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::init_db().await?;
    
    println!("Database connected successfully!");
    
    tokio::signal::ctrl_c().await?;
    Ok(())
}
