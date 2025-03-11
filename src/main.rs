mod db;
mod collector;
mod services;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tera::{Tera, Context};
use tokio_postgres::NoTls;
use std::sync::Arc;
use std::fs;

async fn index(client: web::Data<Arc<tokio_postgres::Client>>, tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    // Obtener las tablas de PostgreSQL
    match client.query("SELECT table_name FROM information_schema.tables WHERE table_schema='public'", &[]).await {
        Ok(rows) => {
            let mut tables = Vec::new();
            for row in rows {
                let table_name: &str = row.get(0);
                tables.push(table_name.to_string());
            }
            context.insert("tables", &tables);
        }
        Err(e) => {
            eprintln!("Error querying tables: {}", e);
            return HttpResponse::InternalServerError().body("Error querying tables");
        }
    }

    match tmpl.render("index.html", &context) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().body("Error rendering template")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectar a la base de datos de manera asíncrona
    let client = db::connect_to_db().await?;
    let client = Arc::new(client);

    // Lanzar el colector de datos
    collector::collect_data(&client).await?;

    // Configurar Actix Web y Tera
    let template_path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
    println!("Loading templates from: {}", template_path);

    // Listar archivos en el directorio de plantillas
    match fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/templates")) {
        Ok(paths) => {
            for path in paths {
                println!("Template file: {}", path.unwrap().path().display());
            }
        }
        Err(e) => {
            println!("Error reading template directory: {}", e);
            ::std::process::exit(1);
        }
    }

    let tera = match Tera::new(template_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates: {}", e);
            ::std::process::exit(1);
        }
    };
    let tera_data = web::Data::new(tera);

    // Iniciar el servidor web
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(tera_data.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    println!("Data Catalog is running!");

    Ok(())
}