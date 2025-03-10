use crate::services::data_source::DataSource;
use crate::db::{insert_or_update_allopathic_data, insert_or_update_train_station};
use crate::services::transport_api::fetch_train_station_data;
use tokio_postgres::Client;

pub async fn collect_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let data_source = DataSource::new();

    // Procesar allopathic.csv
    let allopathic_records = data_source.read_allopathic_csv("drug_data/allopathic.csv")?;
    println!("Collecting data from allopathic...");
    for record in allopathic_records {
        insert_or_update_allopathic_data(client, record.sl, &record.manufacturer, &record.brand_name, &record.generic_name, &record.strength, &record.dosages_description, &record.use_for, &record.dar).await;
    }

    // Procesar datos de TransportAPI
    let app_id = "6ec41647";
    let app_key = "0cc4aba2d857dfb39d0d660880ddce96";
    let station_code = "SJP"; // Ejemplo de código de estación
    match fetch_train_station_data(app_id, app_key, station_code).await {
        Ok(train_station) => {
            //println!("Collecting train station data...");
            insert_or_update_train_station(client, &train_station).await;
        }
        Err(e) => {
            eprintln!("Failed to fetch train station data: {}", e);
        }
    }

    Ok(())
}