mod db;
mod collector;
mod services;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tera::{Tera, Context, Value};
use std::collections::HashMap;
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

async fn table_data(client: web::Data<Arc<tokio_postgres::Client>>, tmpl: web::Data<Tera>, table_name: web::Path<String>) -> impl Responder {
    let mut context = Context::new();
    let table_name = table_name.into_inner();

    // Obtener los datos de la tabla específica
    let query = format!("SELECT * FROM {}", table_name);
    match client.query(&query, &[]).await {
        Ok(rows) => {
            let mut data = Vec::new();
            let mut column_names = Vec::new();

            if let Some(first_row) = rows.get(0) {
                for column in first_row.columns() {
                    column_names.push(column.name().to_string());
                }
            }

            for row in rows {
                let mut row_map = HashMap::new();
                for (i, column_name) in column_names.iter().enumerate() {
                    let value: Value = match row.try_get::<usize, Option<String>>(i) {
                        Ok(Some(val)) => Value::String(val),
                        Ok(None) => Value::String("NULL".to_string()),
                        Err(_) => match row.try_get::<usize, Option<i32>>(i) {
                            Ok(Some(val)) => Value::Number(val.into()),
                            Ok(None) => Value::String("NULL".to_string()),
                            Err(_) => match row.try_get::<usize, Option<i64>>(i) {
                                Ok(Some(val)) => Value::Number(val.into()),
                                Ok(None) => Value::String("NULL".to_string()),
                                Err(_) => match row.try_get::<usize, Option<f64>>(i) {
                                    Ok(Some(val)) => Value::Number(serde_json::Number::from_f64(val).unwrap_or_else(|| serde_json::Number::from(0))),
                                    Ok(None) => Value::String("NULL".to_string()),
                                    Err(_) => Value::String("Error".to_string()),
                                },
                            },
                        },
                    };
                    row_map.insert(column_name.clone(), value);
                }
                data.push(row_map);
            }

            context.insert("table_name", &table_name);
            context.insert("columns", &column_names);
            context.insert("data", &data);
        }
        Err(e) => {
            eprintln!("Error querying table data: {}", e);
            return HttpResponse::InternalServerError().body("Error querying table data");
        }
    }

    match tmpl.render("table.html", &context) {
        Ok(rendered) => {
            println!("Rendered template successfully");
            HttpResponse::Ok().body(rendered)
        },
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
            .route("/table/{table_name}", web::get().to(table_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    println!("Data Catalog is running!");

    Ok(())
}




