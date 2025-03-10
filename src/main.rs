mod db;
mod collector;
mod services;

fn main() {
    // Inicializar la conexión a PostgreSQL
    let _client = db::connect_to_db();

    // Lanzar el colector de datos
    collector::collect_data();

    println!("Data Catalog is running!");
}