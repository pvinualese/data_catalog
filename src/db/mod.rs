use postgres::{Client, NoTls};

pub fn connect_to_db() -> Client {
    Client::connect("host=localhost user=postgres password=secret dbname=data_catalog", NoTls).unwrap()
}

pub fn insert_or_update_allopathic_data(sl: i32, manufacturer: &str, brand_name: &str, generic_name: &str, strength: &str, dosages_description: &str, use_for: &str, dar: &str) {
    let mut client = connect_to_db();
    client.execute(
        "INSERT INTO allopathic (sl, manufacturer, brand_name, generic_name, strength, dosages_description, use_for, dar) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (sl) DO UPDATE SET manufacturer = EXCLUDED.manufacturer, brand_name = EXCLUDED.brand_name, generic_name = EXCLUDED.generic_name, strength = EXCLUDED.strength, dosages_description = EXCLUDED.dosages_description, use_for = EXCLUDED.use_for, dar = EXCLUDED.dar",
        &[&sl, &manufacturer, &brand_name, &generic_name, &strength, &dosages_description, &use_for, &dar],
    ).unwrap();
}

