use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServiceTimetable {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Departure {
    pub mode: String,
    pub service: String,
    pub train_uid: String,
    pub platform: Option<String>,
    pub operator: String,
    pub operator_name: String,
    pub aimed_departure_time: String,
    pub aimed_arrival_time: Option<String>,
    pub aimed_pass_time: Option<String>,
    pub origin_name: String,
    pub destination_name: String,
    pub source: String,
    pub category: String,
    pub service_timetable: ServiceTimetable,
    pub status: String,
    pub expected_arrival_time: Option<String>,
    pub expected_departure_time: Option<String>,
    pub best_arrival_estimate_mins: Option<i32>,
    pub best_departure_estimate_mins: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TrainStationResponse {
    pub date: Option<String>, // Hacer que el campo sea opcional
    pub time_of_day: Option<String>, // Hacer que el campo sea opcional
    pub request_time: Option<String>,
    pub station_name: Option<String>,
    pub station_code: Option<String>, // Hacer que el campo sea opcional
    pub departures: Option<Departures>, // Hacer que el campo sea opcional
}

#[derive(Debug, Deserialize)]
pub struct Departures {
    pub all: Vec<Departure>,
}

pub async fn fetch_train_station_data(app_id: &str, app_key: &str, station_code: &str) -> Result<TrainStationResponse, Box<dyn std::error::Error>> {
    let url = format!("https://transportapi.com/v3/uk/train/station/{}/live.json?app_id={}&app_key={}", station_code, app_id, app_key);
    let response = reqwest::get(&url).await?;
    let response_text = response.text().await?;
    //println!("Response JSON: {}", response_text); // Imprimir la respuesta JSON
    println!("Collecting train station data...");
    let station_data: TrainStationResponse = serde_json::from_str(&response_text).map_err(|e| format!("Error deserializing JSON: {}", e))?;
    //let station_data: TrainStationResponse = response.json().await?;
    Ok(station_data)
}