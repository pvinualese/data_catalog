use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AllopathicRecord {
    #[serde(rename = "SL")]
    pub sl: i32,
    #[serde(rename = "Name of the Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "Brand Name")]
    pub brand_name: String,
    #[serde(rename = "Generic Name")]
    pub generic_name: String,
    #[serde(rename = "Strength")]
    pub strength: String,
    #[serde(rename = "Dosages Description")]
    pub dosages_description: String,
    #[serde(rename = "Use For")]
    pub use_for: String,
    #[serde(rename = "DAR")]
    pub dar: String,
}


pub struct DataSource;

impl DataSource {
    pub fn new() -> Self {
        DataSource
    }

    pub fn read_allopathic_csv<P: AsRef<Path>>(&self, path: P) -> Result<Vec<AllopathicRecord>, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: AllopathicRecord = result?;
            records.push(record);
        }
        Ok(records)
    }

}