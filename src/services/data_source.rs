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

    // toma una ruta de archivo y devuelve un Result que contiene un vector de registros AllopathicRecord o un error
    pub fn read_allopathic_csv<P: AsRef<Path>>(&self, path: P) -> Result<Vec<AllopathicRecord>, Box<dyn Error>> {
        let file = File::open(path)?; //abre el archivo en la ruta especificada
        let mut rdr = csv::Reader::from_reader(file); //crea un lector CSV a partir del archivo
        let mut records = Vec::new(); //crea un vector mutable para almacenar los registros
        for result in rdr.deserialize() {
            // itera sobre los registros deserializados
            let record: AllopathicRecord = result?; //deserializa el registro
            records.push(record); //agrega el registro al vector
        }
        Ok(records) //devuelve el vector de registros
    }

}