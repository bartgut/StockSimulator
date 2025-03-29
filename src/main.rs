use std::{fs, io};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use chrono::NaiveDate;
use itertools::Itertools;
use crate::utils::vec_to_csv::SaveVecToCsv;
use rand::prelude::*;
use rayon::current_num_threads;
use rayon::prelude::*;
use serde::Deserialize;
use crate::brokage::brokage_stocks::get_available_stocks;

use crate::broker_fee::PricePercentageFee;
use crate::grid_search::grid_search::GridSearch;
use crate::grid_search::parameter::Parameter;
use crate::results_statistics::monte_carlo::monte_carlo_simulation;
use crate::results_statistics::profitable_investment::number_of_profitable_investments;
use crate::stock_data_reader::stock_data_reader::{get_ticker_files, read_from_file, StockPriceInfo};
use crate::stop_loss_strategy::{NoStopLoss, PercentageStopLoss};
use crate::strategies::arima::ArimaStrategy;
use crate::strategies::ema_crossover_strategy::EmaCrossoverStrategy;
use crate::strategies::ema_long_term_trend::EmaLongTermTrendStrategy;
use crate::strategies::growing_ema_investing_strategy::GrowingEmaStrategy;
use crate::strategy_simulator::StrategySimulator;
use crate::strategy_simulator::TradeResult::{Buy, Sell, StopLoss, TakeProfit};
use crate::technical_indicator::keltner_channel::KeltnerChannel;
use crate::strategies::macd_divergence_strategy::MACDDivergenceStrategy;
use crate::strategies::macd_strategy::MACDStrategy;
use crate::strategies::rsi_strategy::RsiStrategy;
use crate::take_profit_strategy::{NoTakeProfit, PercentageTakeProfit};
use crate::technical_indicator::ema::Ema;
use crate::technical_indicator::macd::Macd;
use crate::technical_indicator::percent_off_ath::PercentOffAth;


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
mod take_profit_strategy;

fn simulate_ticker(stock_data: &Vec<StockPriceInfo>,
                    buy_ema_length: usize,
                    sell_ema_length: usize,
                    buy_inclination: f32,
                    sell_inclination: f32,
                    stop_loss_param: f32) -> f32 {
     let mut cash_after_last_sell: f32 = 0.0;
     let mut strategy =
         StrategySimulator::new(10000.0f32,
                                NaiveDate::from_ymd(2019, 11, 1),
                                Box::new(EmaLongTermTrendStrategy::new(buy_ema_length, buy_inclination, sell_inclination)), // 20.0, -10.0
                                Box::new(NoTakeProfit),
                                Box::new(PercentageStopLoss::new(stop_loss_param)),
                                Box::new(PricePercentageFee::new(0.0035)));

     for data in stock_data.iter() {
         let operations_performed = strategy.next_today(data);
         for operation_performed in operations_performed.trade_operations {
             match operation_performed {
                 Sell(sell_trade) => {
                     cash_after_last_sell = sell_trade.after_operation_cash
                 }
                 StopLoss(stop_loss_trade) => {
                     cash_after_last_sell = stop_loss_trade.after_operation_cash
                 }
                 TakeProfit(take_profit_trade) => {
                     cash_after_last_sell = take_profit_trade.after_operation_cash
                 }
                 _ => ()
             }
         }
     }
     cash_after_last_sell
 }


fn generate_indicator_data(file_path: &Path) {
    let file_name_str = file_path.file_name().unwrap().to_str().unwrap();
    println!("Generating data for {}", file_name_str);
    let mut stock_data = read_from_file(file_path);

    let mut technical_indicator = PercentOffAth::new();
    let mut data: Vec<(NaiveDate, Vec<f32>)> = vec![];
    for day in stock_data.iter() {
        data.push((day.date, vec![technical_indicator.next(day.high)]))
    }

    data.save_to_csv(format!("ticker_data/{}_keltner.csv", file_name_str).as_str()).unwrap()
}

fn process_ticker(file_path: &Path, start_date: NaiveDate) -> io::Result<f32> {
    let mut cash_after_last_sell: f32 = 0.0;
    let file_name_str = file_path.file_name().unwrap().to_str().unwrap();
    println!("Simulating strategy for {}", file_name_str);
    let mut stock_data = read_from_file(file_path);


    let mut keltner_channel_simulator =
        StrategySimulator::new(10000.0f32,
                               start_date,
                               Box::new(KeltnerChannel::new(20, 3.0)),
                               Box::new(NoTakeProfit),
                               Box::new(PercentageStopLoss::new(0.5)),
                               Box::new(PricePercentageFee::new(0.0035)));

    let mut buy_operation = vec![];
    let mut sell_operation = vec![];
    let mut stop_loss_operation = vec![];
    let mut take_profit_operation = vec![];
    let mut strategy_results: Vec<(NaiveDate, Vec<f32>)> = vec![];
    let mut previous_date: Option<StockPriceInfo> = None;


    for day in stock_data.iter() {
        let result = keltner_channel_simulator.next(day, &previous_date);
        strategy_results.push((result.operation_date, result.strategy_params.today.into()));
        for operation_performed in result.trade_operations {
            match operation_performed {
                Buy(buy_trade) => buy_operation.push((result.operation_date, vec![buy_trade.price])),
                Sell(sell_trade) => {
                    sell_operation.push((result.operation_date, vec![sell_trade.price]));
                    cash_after_last_sell = sell_trade.after_operation_cash
                }
                StopLoss(stop_loss_trade) => {
                    stop_loss_operation.push((result.operation_date, vec![stop_loss_trade.price]));
                    cash_after_last_sell = stop_loss_trade.after_operation_cash
                }
                TakeProfit(take_profit_trade) => {
                    take_profit_operation.push((result.operation_date, vec![take_profit_trade.price]));
                    cash_after_last_sell = take_profit_trade.after_operation_cash
                }
            }
        }

        previous_date = Some(day.clone())
    }
    buy_operation.save_to_csv(format!("ticker_data/signals/{}_keltner_buy_signal.csv", file_name_str).as_str());
    sell_operation.save_to_csv(format!("ticker_data/signals/{}_keltner_sell_signal.csv", file_name_str).as_str());
    stop_loss_operation.save_to_csv(format!("ticker_data/signals/{}_keltner_stop_loss_signal.csv", file_name_str).as_str());
    take_profit_operation.save_to_csv(format!("ticker_data/signals/{}_keltner_take_profit.csv", file_name_str).as_str());
    strategy_results.save_to_csv(format!("ticker_data/{}_keltner.csv", file_name_str).as_str());
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

fn process_directory_data_generation(dir_path: &Path, brokage_house: &str) {
    let files = get_ticker_files(dir_path, brokage_house);

    files.par_iter().for_each(|filepath| {
        generate_indicator_data(filepath)
    })
}

fn process_directory_v2(data: &HashMap<String, Vec<StockPriceInfo>>,
                           buy_ema_length: usize,
                           sell_ema_length: usize,
                           buy_inclination: f32,
                           sell_inclination: f32,
                           stop_loss_param: f32) -> HashMap<String, f32> {
    let mut result_map: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));

    data.par_iter().for_each(|(file_name, stock_data)| {
        let result = simulate_ticker(
            stock_data, buy_ema_length, sell_ema_length, buy_inclination, sell_inclination, stop_loss_param);
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
    //grid_search_growing_ema();
    //process_directory_data_generation(Path::new("nasdaq"), "XTB");

    let map = process_directory(Path::new("nasdaq"), "XTB", NaiveDate::from_ymd(2019, 11, 1));
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
    monte_carlo_result.save_to_csv("monte_carlo_simulation.csv");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}

fn grid_search_growing_ema() -> f32 {
    let buy_ema_length_param = Parameter::new(70.0, 200.0, 1.0);
    let sell_ema_length_param = Parameter::new(1.0, 1.0, 1.0);
    let buy_inclination_param = Parameter::new(0.0, 0.2, 0.01);
    let sell_inclination_param = Parameter::new(0.0, 0.2, 0.01);
    let stop_loss_param = Parameter::new(0.1, 0.5, 0.05);

    let search = GridSearch::new(
        vec![buy_ema_length_param, sell_ema_length_param, buy_inclination_param, sell_inclination_param, stop_loss_param]);

    let files = get_ticker_files(Path::new("nasdaq"), "XTB");
    let loaded_files: HashMap<String, Vec<StockPriceInfo>> =
        files.par_iter()
            .map(|file_path| (file_path.file_name().unwrap().to_str().unwrap().to_ascii_lowercase(), read_from_file(file_path)))
            .collect();

    let strategy = |params: &[f32]| -> f32 {
        let buy_ema_length: usize = params[0] as usize;
        let sell_ema_length: usize = params[1] as usize;
        let buy_inclination: f32 = params[2];
        let sell_inclination: f32 = params[3];
        let stop_loss_param: f32 = params[4];

        let res = process_directory_v2(&loaded_files,
                                       buy_ema_length,
                                       sell_ema_length,
                                       buy_inclination,
                                       sell_inclination,
                                       stop_loss_param);
        number_of_profitable_investments(res.values().cloned().collect(), 10000.0) as f32
    };

    println!("Starting grid search");
    let results = search.search(strategy);
    results.save_to_csv("growing_ema_grid_search.csv");
    0.0

}
