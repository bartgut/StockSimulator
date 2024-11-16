use std::{fs, io};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
use crate::grid_search::grid_search::GridSearch;
use crate::grid_search::parameter::Parameter;
use crate::results_statistics::monte_carlo::monte_carlo_simulation;
use crate::stock_data_reader::stock_data_reader::{get_ticker_files, read_from_file, StockPriceInfo};
use crate::stop_loss_strategy::PercentageStopLoss;
use crate::strategies::growing_ema_investing_strategy::GrowingEmaStrategy;
use crate::strategy_simulator::StrategySimulator;
use crate::strategy_simulator::TradeResult::{Buy, Sell, StopLoss};
use crate::technical_indicator::keltner_channel::KeltnerChannel;
use crate::strategies::macd_divergence_strategy::MACDDivergenceStrategy;
use crate::strategies::macd_strategy::MACDStrategy;
use crate::strategies::rsi_strategy::RsiStrategy;
use crate::technical_indicator::ema::Ema;
use crate::technical_indicator::macd::Macd;


mod strategy_simulator;
mod stop_loss_strategy;
mod broker_fee;
mod technical_indicator;
mod strategies;
mod serde_serialization;
mod results_statistics;
mod utils;
mod brokage;
mod grid_search;
mod stock_data_reader;

 fn simulate_ticker(stock_data: &Vec<StockPriceInfo>,
                    buy_ema_length: usize,
                    sell_ema_length: usize,
                    buy_inclination: f32,
                    sell_inclination: f32) -> f32 {
     let mut cash_after_last_sell: f32 = 0.0;
     let mut strategy =
         StrategySimulator::new(10000.0f32,
                                 NaiveDate::from_ymd(2019, 11, 1),
                                 Box::new(GrowingEmaStrategy::with_separate_buy_sell_ema(buy_ema_length, sell_ema_length, buy_inclination, sell_inclination)), // 20.0, -10.0
                                 Box::new(PercentageStopLoss::new(0.1)),
                                 Box::new(PricePercentageFee::new(0.0035)));

     for data in stock_data.iter() {
         let operations_performed = strategy.next_today(data);
         for operation_performed in operations_performed {
             match operation_performed {
                 Sell(sell_trade) => {
                     cash_after_last_sell = sell_trade.after_operation_cash
                 }
                 _ => ()
             }
         }
     }
     cash_after_last_sell
 }


fn process_ticker(file_path: &Path, start_date: NaiveDate) -> io::Result<f32> {
    let mut cash_after_last_sell: f32 = 0.0;
    let file_name_str = file_path.file_name().unwrap().to_str().unwrap();
    println!("Simulating strategy for {}", file_name_str);
    let mut stock_data = read_from_file(file_path);


    let mut keltner_channel_simulator =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(KeltnerChannel::new(20, 2.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));


    let mut growing_ema_simulator =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(GrowingEmaStrategy::with_separate_buy_sell_ema(45, 45, 0.0, -10.0)), // 20.0, -10.0
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut rsi_strategy =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(RsiStrategy::new(13, 30.0, 70.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut macd_strategy_simulator =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(MACDStrategy::default()),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut macd_divirgence_simulator =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(MACDDivergenceStrategy::default()),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut keltner_channel = KeltnerChannel::new(20, 2.0);
    let mut macd = Macd::default();
    let mut ema = Ema::new(45);

    let mut ema_line_vec = vec![];
    //let mut lower_band_vec = vec![];
    //let mut upper_band_vec = vec![];
    let mut buy_operation = vec![];
    let mut sell_operation = vec![];
    let mut stop_loss_operation = vec![];
    let mut previous_date: Option<StockPriceInfo> = None;


    for (index, day) in stock_data.iter().enumerate() {
        let keltner_channel_result =
            keltner_channel.next(day.close, day.high, day.low, previous_date.clone().map(|u| u.close).unwrap_or(0.0f32));
        let macd_result =
            macd.next(day.close);
        let ema_result = ema.next(day.close);

        //let operations_performed = macd_divirgence_simulator.next(day, &previous_date);
        //let operations_performed = macd_strategy_simulator.next(day, &previous_date);
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

        //ema_line_vec.push(macd_result.macd_line);
        //upper_band_vec.push(macd_result.signal_line);
        //lower_band_vec.push(macd_result.signal_line);

        //ema_line_vec.push(keltner_channel_result.ema);
        //upper_band_vec.push(keltner_channel_result.upper_band);
        //lower_band_vec.push(keltner_channel_result.lower_band);

        ema_line_vec.push(ema_result);
        previous_date = Some(day.clone())
    }
    let chart = Chart::new()
        .title(Title::new().top("Ticker"))
        .legend(Legend::new().top("bottom"))
        .x_axis(Axis::new().type_(AxisType::Category))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(stock_data.iter().map(|x| x.close).collect()))
        .series(Line::new().data(ema_line_vec))
        //.series(Line::new().line_style(LineStyle::new().color("blue")).data(upper_band_vec))
        //.series(Line::new().line_style(LineStyle::new().color("blue")).data(lower_band_vec))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("green")).symbol_size(20).data(buy_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("red")).symbol_size(20).data(sell_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("yellow")).symbol_size(20).data(stop_loss_operation));


    let mut renderer = ImageRenderer::new(5000, 4000);
    let res = renderer.save(&chart, format!("ticker_images/{}.svg", file_name_str));
    Ok(cash_after_last_sell)
}

fn process_directory(dir_path: &Path, brokage_house: &str, start_date: NaiveDate) -> HashMap<String, f32> {
    let mut result_map: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));
    let files = get_ticker_files(dir_path, brokage_house);

    files.par_iter().for_each(|filepath| {
        let result = process_ticker(filepath, start_date);
        let mut map_unlocked = result_map.lock().unwrap();
        let file_name = filepath.file_name().unwrap().to_str().unwrap();
        map_unlocked.insert(file_name.to_ascii_lowercase(), result.unwrap());
    });

    Arc::try_unwrap(result_map)
        .expect("sth went wrong")
        .into_inner()
        .unwrap()
}

fn process_directory_v2(data: &HashMap<String, Vec<StockPriceInfo>>,
                           buy_ema_length: usize,
                           sell_ema_length: usize,
                           buy_inclination: f32,
                           sell_inclination: f32) -> HashMap<String, f32> {
    let mut result_map: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));

    data.par_iter().for_each(|(file_name, stock_data)| {
        let result = simulate_ticker(
            stock_data, buy_ema_length, sell_ema_length, buy_inclination, sell_inclination);
        let mut map_unlocked = result_map.lock().unwrap();
        map_unlocked.insert(file_name.clone(), result);
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
    let start = Instant::now();
    grid_search_growing_ema();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    /*let map = process_directory(Path::new("nasdaq"), "XTB", NaiveDate::from_ymd(2019, 11, 1));
    let mut vec_tuple: Vec<(String, f32)> = map.into_iter().collect();
    vec_tuple.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    for (ticker, accumulated_cash) in vec_tuple.iter() {
        println!("Ticker: {} - {}", ticker, accumulated_cash)
    }
    let gained_cash = vec_tuple.iter().filter(|&value| value.1 > 10000.0).count();
    let no_data = vec_tuple.iter().filter(|&value| value.1 == 0.0).count();
    let lost_cash = vec_tuple.iter().filter(|&value| value.1 < 10000.0 && value.1 != 0.0).count();
    println!("Cash gained in {} tickers", gained_cash);
    println!("Cash lost in {} tickers", lost_cash);
    println!("No buy/sell operation in {} tickers", no_data);
    let ROIs: Vec<f32> = vec_tuple.iter().map(|x| x.1).collect();
    let monte_carlo_result = monte_carlo_simulation(ROIs, 20000, 5);
    monte_carlo_result.save_to_csv("monte_carlo_simulation.csv"); */
    Ok(())
}

fn grid_search_growing_ema() -> f32 {
    let buy_ema_length_param = Parameter::new(1.0, 75.0, 1.0);
    let sell_ema_length_param = Parameter::new(1.0, 75.0, 1.0);
    let buy_inclination_param = Parameter::new(0.0, 20.0, 1.0);
    let sell_inclination_param = Parameter::new(-20.0, 0.0, 1.0);

    let search = GridSearch::new(
        vec![buy_ema_length_param, sell_ema_length_param, buy_inclination_param, sell_inclination_param]);

    let files = get_ticker_files(Path::new("nasdaq"), "XTB");
    let loaded_files: HashMap<String, Vec<StockPriceInfo>> =
        files.par_iter()
            .map(|file_path| (file_path.file_name().unwrap().to_str().unwrap().to_ascii_lowercase(), read_from_file(file_path)))
            .collect();

    let strategy = |params: &[f32]| -> f32 {
        let buy_ema_length: usize= params[0] as usize;
        let sell_ema_length: usize = params[1] as usize;
        let buy_inclination: f32 = params[2];
        let sell_inclination: f32 = params[3];

        let res = process_directory_v2(&loaded_files,
                                       buy_ema_length,
                                       sell_ema_length,
                                       buy_inclination,
                                       sell_inclination);
        let sum: f32 = res.values().sum();
        let count = res.len() as f32;
        sum / count
    };

    let results = search.search(strategy);
    results.save_to_csv("growing_ema_grid_search.csv");
    0.0

}
