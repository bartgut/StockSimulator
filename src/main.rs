use std::error::Error;

use charming::{Chart, ImageRenderer};
use charming::component::{Axis, Legend, Title};
use charming::element::{AxisType, LineStyle, LineStyleType};
use charming::series::Bar;
use charming::series::Line;
use crate::broker_fee::PricePercentageFee;

use crate::stop_loss_strategy::{NoStopLoss, PercentageStopLoss};
use crate::strategy_simulator::StrategySimulator;
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
        .from_path("dell.us.txt")?;


    for record in reader.deserialize() {
        let stock_price_info: StockPriceInfo = record?;
        stock_date.push(stock_price_info)
    }

    /*let mut macd = Macd::new(12, 26, 9);
    let mut keltner_channel = KeltnerChannel::new(20); */
    let mut simulator =
        StrategySimulator::new(10000.0f32,
                               Box::new(KeltnerChannel::new(20, 2.0)),
                               Box::new(PercentageStopLoss::new(0.1)),
                               Box::new(PricePercentageFee::new(0.0035)));

    /*let mut macd_fast_line_vec = vec![];
    let mut macd_signal_line_vec = vec![];
    let mut macd_histogram = vec![];
    let mut ema_line_vec = vec![];
    let mut lower_band_vec = vec![];
    let mut upper_band_vec = vec![];*/
    let mut previous_date: Option<StockPriceInfo> = None;

    for day in stock_date.iter() {
        //let macd_result = macd.next(day.close);
        //let keltner_channel_result =
        //    keltner_channel.next(day.close, day.high, day.low, previous_date.clone().map(|u| u.close).unwrap_or(0.0f32));
        simulator.next(day, &previous_date);
        /*ema_line_vec.push(keltner_channel_result.ema);
        upper_band_vec.push(keltner_channel_result.upper_band);
        lower_band_vec.push(keltner_channel_result.lower_band);
        macd_signal_line_vec.push(macd_result.signal_line);
        macd_fast_line_vec.push(macd_result.macd_line);
        macd_histogram.push(macd_result.macd_line - macd_result.signal_line);*/
        previous_date = Some(day.clone())
    }
    /*let chart = Chart::new()
        .title(Title::new().top("ZEP ticker"))
        .legend(Legend::new().top("bottom"))
        .x_axis(Axis::new().type_(AxisType::Category))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(stock_date.iter().map(|x| x.close).collect()))
        .series(Line::new().data(ema_line_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data( upper_band_vec))
        .series(Line::new().line_style(LineStyle::new().color("blue")).data(lower_band_vec))
        .series(Line::new().line_style(LineStyle::new().color("green").type_(LineStyleType::Dashed)).data(macd_signal_line_vec))
        .series(Line::new().line_style(LineStyle::new().color("red")).data(macd_fast_line_vec))
        .series(Bar::new().data(macd_histogram));

    let mut renderer = ImageRenderer::new(5000, 4000);
    let res = renderer.save(&chart, "zep.svg");
    println!("{:?}", res); */
    Ok(())
}