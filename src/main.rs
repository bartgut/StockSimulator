use std::error::Error;

use charming::{Chart, ImageRenderer, series};
use charming::component::{Axis, Legend, Title};
use charming::element::{AxisType, ItemStyle, LineStyle};
use charming::series::Line;

use crate::broker_fee::PricePercentageFee;
use crate::stop_loss_strategy::PercentageStopLoss;
use crate::strategy_simulator::StrategySimulator;
use crate::strategy_simulator::TradeResult::{Buy, Sell, StopLoss};
use crate::technical_analysis::ema::Ema;
use crate::technical_analysis::keltner_channel::KeltnerChannel;

mod strategy_simulator;
mod stop_loss_strategy;
mod broker_fee;
mod technical_analysis;
mod strategies;

#[derive(Debug, serde::Deserialize, Clone)]
struct StockPriceInfo {
    #[serde(rename = "<TICKER>")]
    ticker: String,
    #[serde(rename = "<PER>")]
    per: String,
    #[serde(rename = "<DATE>")]
    date: String,
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut stock_date = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path("crsp.us.txt")?;


    for record in reader.deserialize() {
        let stock_price_info: StockPriceInfo = record?;
        stock_date.push(stock_price_info)
    }

    let mut keltner_channel = KeltnerChannel::new(20, 2.0);
    let mut keltner_simulator =
        StrategySimulator::new(10000.0f32,
                               Box::new(KeltnerChannel::new(20, 2.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));
    let mut growing_ema_simulator =
        StrategySimulator::new(10000.0f32,
                               Box::new(Ema::new(40)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));




    let mut ema_line_vec = vec![];
    let mut lower_band_vec = vec![];
    let mut upper_band_vec = vec![];
    let mut buy_operation = vec![];
    let mut sell_operation = vec![];
    let mut stop_loss_operation = vec![];
    let mut previous_date: Option<StockPriceInfo> = None;

    for (index, day) in stock_date.iter().enumerate() {
        let keltner_channel_result =
            keltner_channel.next(day.close, day.high, day.low, previous_date.clone().map(|u| u.close).unwrap_or(0.0f32));

        //let operations_performed = growing_ema_simulator.next(day, &previous_date);
        let operations_performed = keltner_simulator.next(day, &previous_date);
        for operation_performed in operations_performed {
            match operation_performed {
                Buy(buy_trade) => buy_operation.push(vec![index as f32, buy_trade.price]),
                Sell(sell_trade) => sell_operation.push(vec![index as f32, sell_trade.price]),
                StopLoss(stop_loss_trade) => stop_loss_operation.push(vec![index as f32, stop_loss_trade.price])
            }
        }

        ema_line_vec.push(keltner_channel_result.ema);
        upper_band_vec.push(keltner_channel_result.upper_band);
        lower_band_vec.push(keltner_channel_result.lower_band);
        previous_date = Some(day.clone())
    }
    let chart = Chart::new()
        .title(Title::new().top("DELL ticker"))
        .legend(Legend::new().top("bottom"))
        .x_axis(Axis::new().type_(AxisType::Category))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(stock_date.iter().map(|x| x.close).collect()))
        .series(Line::new().data(ema_line_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data(upper_band_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data(lower_band_vec))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("green")).symbol_size(20).data(buy_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("red")).symbol_size(20).data(sell_operation))
        .series(series::Scatter::new().item_style(ItemStyle::new().color("yellow")).symbol_size(20).data(stop_loss_operation));


    let mut renderer = ImageRenderer::new(5000, 4000);
    let res = renderer.save(&chart, "zep.svg");
    println!("{:?}", res);
    Ok(())
}