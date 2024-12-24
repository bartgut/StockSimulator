use tokio::runtime::Runtime;
use tonic::transport::Channel;
use crate::stock_data_reader::stock_data_reader::StockPriceInfo;
use crate::strategies::arima::hello_world::arima_service_client::ArimaServiceClient;
use crate::strategies::arima::hello_world::ForecastRequest;
use crate::strategy_simulator::InvestingStrategy;

pub mod hello_world {
    tonic::include_proto!("arima_connector");
}

pub struct ArimaStrategy {
    tokyo_runtime: Runtime,
    history: Vec<f64>,
    client: ArimaServiceClient<Channel>
}

pub struct ArimaResult {
    forecast: f32
}

impl ArimaStrategy {
    pub fn new() -> ArimaStrategy {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let client = runtime.block_on(async {
            ArimaServiceClient::connect("http://[::1]:50051").await
        }).expect("Fatal error");

        Self {
            tokyo_runtime: runtime,
            history: vec![],
            client
        }
    }
}

impl InvestingStrategy<ArimaResult> for ArimaStrategy {
    fn calculation(&mut self, stock_price_info: &StockPriceInfo, yesterday: &Option<StockPriceInfo>) -> ArimaResult {
        self.history.push(stock_price_info.close as f64);
        let res = self.tokyo_runtime.block_on(
            async {
                self.client.forecast(ForecastRequest {
                    time_series: self.history.iter().cloned().collect(),
                    steps: 1
                }).await
            }
        );
        let res_expected = res.expect("Fatal error");

        if let Some(forecast) = res_expected.into_inner().forecast.get(0) {
            ArimaResult {
                forecast: forecast.clone() as f32
            }
        } else {
            ArimaResult {
                forecast: 0.0
            }
        }
    }

    fn buy_signal(&self, stock_price_info: &StockPriceInfo, indicator: &ArimaResult) -> Option<f32> {
        None
    }

    fn sell_signal(&self, stock_price_info: &StockPriceInfo, indicator: &ArimaResult) -> Option<f32> {
        None
    }
}