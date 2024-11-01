use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn get_available_stocks(brokage_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file_path = format!("brokage_house_available_stocks/{}.csv", brokage_name);

    let file = File::open(&file_path)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let available_stocks: Vec<String> = reader.records()
        .filter_map(|result| result.ok())
        .map(|record| record[0].to_string())
        .collect();

    Ok(available_stocks)
}