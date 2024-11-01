use std::{fs, io};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use charming::{Chart, ImageRenderer, series};
use charming::component::{Axis, Legend, Title};
use charming::element::{AxisType, ItemStyle, LineStyle};
use charming::series::Line;
use chrono::NaiveDate;
use itertools::Itertools;
use crate::utils::vec_to_csv::SaveVecToCsv;
use rand::prelude::*;
use rayon::prelude::*;
use serde::Deserialize;
use crate::brokage::brokage_stocks::get_available_stocks;

use crate::broker_fee::PricePercentageFee;
use crate::results_statistics::monte_carlo::monte_carlo_simulation;
use crate::stop_loss_strategy::PercentageStopLoss;
use crate::strategies::growing_ema_investing_strategy::GrowingEmaStrategy;
use crate::strategy_simulator::StrategySimulator;
use crate::strategy_simulator::TradeResult::{Buy, Sell, StopLoss};
use crate::technical_indicator::keltner_channel::KeltnerChannel;
use crate::serde_serialization::naive_date_yyyymmdd_format::naive_date_yyyymmdd_format;
use crate::strategies::rsi_strategy::RsiStrategy;


mod strategy_simulator;
mod stop_loss_strategy;
mod broker_fee;
mod technical_indicator;
mod strategies;
mod serde_serialization;
mod results_statistics;
mod utils;
mod brokage;

#[derive(Debug, serde::Deserialize, Clone)]
struct StockPriceInfo {
    #[serde(rename = "<TICKER>")]
    ticker: String,
    #[serde(rename = "<PER>")]
    per: String,
    #[serde(rename = "<DATE>", with = "naive_date_yyyymmdd_format")]
    date: NaiveDate,
    #[serde(rename = "<TIME>")]
    time: String,
    #[serde(rename = "<OPEN>")]
    open: f32,
    #[serde(rename = "<HIGH>")]
    high: f32,
    #[serde(rename = "<LOW>")]
    low: f32,
    #[serde(rename = "<CLOSE>")]
    close: f32,
    #[serde(rename = "<VOL>")]
    vol: f32,
    #[serde(rename = "<OPENINT>")]
    openint: u32
}


fn process_ticker(file_path: &Path) -> io::Result<f32> {
    let mut cash_after_last_sell: f32 = 0.0;
    let file_name_str = file_path.file_name().unwrap().to_str().unwrap();
    println!("Simulating strategy for {}", file_name_str);
    let mut stock_data = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(file_path)?;

    for record in reader.deserialize() {
        let stock_price_info: StockPriceInfo = record?;
        stock_data.push(stock_price_info)
    }


    let mut keltner_channel_simulator =
        StrategySimulator::new(10000.0f32,
                               Box::new(KeltnerChannel::new(20, 2.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));


    let mut growing_ema_simulator =
        StrategySimulator::new(10000.0f32,
                               Box::new(GrowingEmaStrategy::new(20, 20.0, -10.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut rsi_strategy =
        StrategySimulator::new(10000.0f32,
                               Box::new(RsiStrategy::new(13, 30.0, 70.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut keltner_channel = KeltnerChannel::new(20, 2.0);

    let mut ema_line_vec = vec![];
    let mut lower_band_vec = vec![];
    let mut upper_band_vec = vec![];
    let mut buy_operation = vec![];
    let mut sell_operation = vec![];
    let mut stop_loss_operation = vec![];
    let mut previous_date: Option<StockPriceInfo> = None;


    for (index, day) in stock_data.iter().enumerate() {
        let keltner_channel_result =
            keltner_channel.next(day.close, day.high, day.low, previous_date.clone().map(|u| u.close).unwrap_or(0.0f32));

        //let operations_performed = growing_ema_simulator.next(day, &previous_date);
        //let operations_performed = keltner_channel_simulator.next(day, &previous_date);
        let operations_performed = growing_ema_simulator.next(day, &previous_date);
        for operation_performed in operations_performed {
            match operation_performed {
                Buy(buy_trade) => buy_operation.push(vec![index as f32, buy_trade.price]),
                Sell(sell_trade) => {
                    sell_operation.push(vec![index as f32, sell_trade.price]);
                    cash_after_last_sell = sell_trade.after_operation_cash
                }
                StopLoss(stop_loss_trade) => stop_loss_operation.push(vec![index as f32, stop_loss_trade.price])
            }
        }

        ema_line_vec.push(keltner_channel_result.ema);
        upper_band_vec.push(keltner_channel_result.upper_band);
        lower_band_vec.push(keltner_channel_result.lower_band);
        previous_date = Some(day.clone())
    }
    let chart = Chart::new()
        .title(Title::new().top("Ticker"))
        .legend(Legend::new().top("bottom"))
        .x_axis(Axis::new().type_(AxisType::Category))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(stock_data.iter().map(|x| x.close).collect()))
        .series(Line::new().data(ema_line_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data(upper_band_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data(lower_band_vec))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("green")).symbol_size(20).data(buy_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("red")).symbol_size(20).data(sell_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("yellow")).symbol_size(20).data(stop_loss_operation));


    let mut renderer = ImageRenderer::new(5000, 4000);
    let res = renderer.save(&chart, format!("ticker_images/{}.svg", file_name_str));
    Ok(cash_after_last_sell)
}

fn process_directory(dir_path: &Path, brokage_house: &str) -> HashMap<String, f32> {
    let mut result_map: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));
    let brokage_house_available_stocks = get_available_stocks(brokage_house).unwrap();

    let files: Vec<_> = fs::read_dir(dir_path).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .filter(|entry| brokage_house_available_stocks.iter().any(|ticker_name| entry.ends_with(format!("{}.txt", ticker_name.to_lowercase()))))
        .collect();

    files.par_iter().for_each(|filepath| {
        let result = process_ticker(filepath);
        let mut map_unlocked = result_map.lock().unwrap();
        let file_name = filepath.file_name().unwrap().to_str().unwrap();
        map_unlocked.insert(file_name.to_ascii_lowercase(), result.unwrap());
    });

    Arc::try_unwrap(result_map)
        .expect("sth went wrong")
        .into_inner()
        .unwrap()
}

fn bucket_values(values: Vec<f32>, window: f32) -> HashMap<i32, Vec<f32>> {
    values
        .into_iter()
        .into_group_map_by(|&value| ((value/window).floor() as i32) * window as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = process_directory(Path::new("nasdaq"), "XTB");
    let mut vec_tuple: Vec<(String, f32)> = map.into_iter().collect();
    vec_tuple.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    for (ticker, accumulated_cash) in vec_tuple.iter() {
        println!("Ticker: {} - {}", ticker, accumulated_cash)
    }
    let gained_cash = vec_tuple.iter().filter(|&value| value.1 > 10000.0).count();
    let no_data = vec_tuple.iter().filter(|&value| value.1 == 0.0).count();
    let lost_cash = vec_tuple.iter().filter(|&value| value.1 < 10000.0 && value.1 != 0.0).count();
    println!("Cash gained in {} test_tickers", gained_cash);
    println!("Cash lost in {} test_tickers", lost_cash);
    println!("No buy/sell operation in {} test_tickers", no_data);
    let ROIs: Vec<f32> = vec_tuple.iter().map(|x| x.1).collect();
    let monte_carlo_result = monte_carlo_simulation(ROIs, 20000, 5);
    monte_carlo_result.save_to_csv("monte_carlo_simulation.csv");
    Ok(())
}
