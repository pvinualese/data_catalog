mod db;
mod collector;
mod services;

use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectar a la base de datos de manera asíncrona
    let client = db::connect_to_db().await?;

    // Lanzar el colector de datos
    collector::collect_data(&client).await?;

    println!("Data Catalog is running!");

    Ok(())
}