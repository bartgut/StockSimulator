mod macd;
mod ema;

use std::error::Error;
use charming::{Chart, ImageRenderer};
use charming::component::{Axis, Legend, Title};
use charming::element::{AxisType, LineStyle, LineStyleType};
use charming::series::Line;
use crate::ema::Ema;
use crate::macd::Macd;

#[derive(Debug, serde::Deserialize)]
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
    vol: u32,
    #[serde(rename = "<OPENINT>")]
    openint: u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stock_date = vec![];
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path("zep.txt")?;


    for record in reader.deserialize() {
        let stock_price_info: StockPriceInfo = record?;
        stock_date.push(stock_price_info)
    }

    let mut macd = Macd::new(12, 26, 9);
    //stock_date.reverse();
    //stock_date.truncate(60);
    //stock_date.reverse();

    let mut macd_fast_line_vec = vec![];
    let mut macd_signal_line_vec = vec![];

    for price in stock_date.iter().map(|x| x.close) {
        let (fast_res, signal_res) = macd.next(price);
        macd_signal_line_vec.push(signal_res);
        macd_fast_line_vec.push(fast_res)
    }
    let chart = Chart::new()
        .title(Title::new().top("ZEP ticker"))
        .legend(Legend::new().top("bottom"))
        .x_axis(Axis::new().type_(AxisType::Category))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(stock_date.iter().map(|x| x.close).collect()))
        .series(Line::new().line_style(LineStyle::new().color("green").type_(LineStyleType::Dashed)).data(macd_signal_line_vec))
        .series(Line::new().line_style(LineStyle::new().color("red")).data(macd_fast_line_vec));

    let mut renderer = ImageRenderer::new(5000, 4000);
    let res = renderer.save(&chart, "zep.svg");
    println!("{:?}", res);
    Ok(())
}
