use std::fs;
use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use crate::brokage::brokage_stocks::get_available_stocks;
use crate::serde_serialization::naive_date_yyyymmdd_format::naive_date_yyyymmdd_format;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct StockPriceInfo {
    #[serde(rename = "<TICKER>")]
    pub ticker: String,
    #[serde(rename = "<PER>")]
    pub per: String,
    #[serde(rename = "<DATE>", with = "naive_date_yyyymmdd_format")]
    pub date: NaiveDate,
    #[serde(rename = "<TIME>")]
    pub time: String,
    #[serde(rename = "<OPEN>")]
    pub open: f32,
    #[serde(rename = "<HIGH>")]
    pub high: f32,
    #[serde(rename = "<LOW>")]
    pub low: f32,
    #[serde(rename = "<CLOSE>")]
    pub close: f32,
    #[serde(rename = "<VOL>")]
    pub vol: f32,
    #[serde(rename = "<OPENINT>")]
    pub openint: u32
}

pub fn read_from_file(file_path: &Path) -> Vec<StockPriceInfo> {
    let mut stock_data = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(file_path)
        .unwrap();

    for record in reader.deserialize() {
        let stock_price_info: StockPriceInfo = record.unwrap();
        stock_data.push(stock_price_info)
    }
    stock_data
}

pub fn get_ticker_files(stock_market: &Path, brokage_house: &str) -> Vec<PathBuf> {
    let brokage_house_available_stocks = get_available_stocks(brokage_house).unwrap();

    fs::read_dir(stock_market).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .filter(|entry| brokage_house_available_stocks.iter().any(|ticker_name| entry.ends_with(format!("{}.txt", ticker_name.to_lowercase()))))
        .collect::<Vec<PathBuf>>()
}