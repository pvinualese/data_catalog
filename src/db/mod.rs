use tokio_postgres::{Client, NoTls};
use crate::services::transport_api::{Departure, TrainStationResponse};
use tokio_postgres::Error;


pub async fn connect_to_db() -> Result<Client, Error> {
    // Se conecta a la base de datos PostgreSQL y devuelve un cliente
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=secret dbname=data_catalog", NoTls).await?;

    // Spawn the connection on the Tokio runtime
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn insert_or_update_allopathic_data(client: &Client, sl: i32, manufacturer: &str, brand_name: &str, generic_name: &str, strength: &str, dosages_description: &str, use_for: &str, dar: &str) -> Result<(), Error> {
    // Query SQL para insertar o actualizar un registro en la tabla allopathic
    client.execute(
        "INSERT INTO allopathic (sl, manufacturer, brand_name, generic_name, strength, dosages_description, use_for, dar) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (sl) DO UPDATE SET manufacturer = EXCLUDED.manufacturer, brand_name = EXCLUDED.brand_name, generic_name = EXCLUDED.generic_name, strength = EXCLUDED.strength, dosages_description = EXCLUDED.dosages_description, use_for = EXCLUDED.use_for, dar = EXCLUDED.dar",
        &[&sl, &manufacturer, &brand_name, &generic_name, &strength, &dosages_description, &use_for, &dar],
    ).await?;
    Ok(())
}

pub async fn insert_or_update_departure(client: &Client, departure: &Departure, station_code: &str) -> Result<(), Error> {
    client.execute(
        "INSERT INTO departures (service, station_code, mode, train_uid, platform, operator, operator_name, aimed_departure_time, aimed_arrival_time, aimed_pass_time, origin_name, destination_name, source, category, service_timetable_id, status, expected_arrival_time, expected_departure_time, best_arrival_estimate_mins, best_departure_estimate_mins) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
        ON CONFLICT (service) DO UPDATE SET station_code = EXCLUDED.station_code, mode = EXCLUDED.mode, train_uid = EXCLUDED.train_uid, platform = EXCLUDED.platform, operator = EXCLUDED.operator, operator_name = EXCLUDED.operator_name, aimed_departure_time = EXCLUDED.aimed_departure_time, aimed_arrival_time = EXCLUDED.aimed_arrival_time, aimed_pass_time = EXCLUDED.aimed_pass_time, origin_name = EXCLUDED.origin_name, destination_name = EXCLUDED.destination_name, source = EXCLUDED.source, category = EXCLUDED.category, service_timetable_id = EXCLUDED.service_timetable_id, status = EXCLUDED.status, expected_arrival_time = EXCLUDED.expected_arrival_time, expected_departure_time = EXCLUDED.expected_departure_time, best_arrival_estimate_mins = EXCLUDED.best_arrival_estimate_mins, best_departure_estimate_mins = EXCLUDED.best_departure_estimate_mins",
        &[&departure.service, &station_code, &departure.mode, &departure.train_uid, &departure.platform, &departure.operator, &departure.operator_name, &departure.aimed_departure_time, &departure.aimed_arrival_time, &departure.aimed_pass_time, &departure.origin_name, &departure.destination_name, &departure.source, &departure.category, &departure.service_timetable.id, &departure.status, &departure.expected_arrival_time, &departure.expected_departure_time, &departure.best_arrival_estimate_mins, &departure.best_departure_estimate_mins],
    ).await?;
    Ok(())
}

pub async fn insert_or_update_train_station(client: &Client, station: &TrainStationResponse) -> Result<(), Error> {
    if let Some(station_code) = &station.station_code {
        client.execute(
            "INSERT INTO train_stations (station_code, station_name, date, time_of_day, request_time) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (station_code) DO UPDATE SET station_name = EXCLUDED.station_name, date = EXCLUDED.date, time_of_day = EXCLUDED.time_of_day, request_time = EXCLUDED.request_time",
            &[&station_code, &station.station_name, &station.date, &station.time_of_day, &station.request_time],
        ).await?;
        if let Some(departures) = &station.departures {
            for departure in &departures.all {
                insert_or_update_departure(client, departure, station_code).await?;
            }
        }
    }
    Ok(())
}

