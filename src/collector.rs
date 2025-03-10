use crate::services::data_source::DataSource;
use crate::db::insert_or_update_allopathic_data;

pub fn collect_data() {
    let data_source = DataSource::new();

    // Procesar allopathic.csv
    let allopathic_records = data_source.read_allopathic_csv("drug_data/allopathic.csv").expect("Failed to read allopathic CSV"); //leer el archivo CSV y almacenar los registros en un vector
    // para cada registro lo inserta en la bbdd
    for record in allopathic_records {
        println!("Collected allopathic data: {:?}", record);
        insert_or_update_allopathic_data(record.sl, &record.manufacturer, &record.brand_name, &record.generic_name, &record.strength, &record.dosages_description, &record.use_for, &record.dar);
    }

}