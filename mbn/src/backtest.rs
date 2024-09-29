use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct BacktestData {
    pub backtest_id: Option<u16>,
    pub backtest_name: String,
    pub parameters: Parameters,
    pub static_stats: StaticStats,
    pub period_timeseries_stats: Vec<PeriodTimeseriesStats>,
    pub daily_timeseries_stats: Vec<DailyTimeseriesStats>,
    pub trades: Vec<Trades>,
    pub signals: Vec<Signals>,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct Parameters {
    pub strategy_name: String,
    pub capital: f64,
    pub schema: String,
    pub data_type: String,
    pub train_start: i64,
    pub train_end: i64,
    pub test_start: i64,
    pub test_end: i64,
    pub tickers: serde_json::Value,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct StaticStats {
    pub total_trades: i32,
    pub total_winning_trades: i32,
    pub total_losing_trades: i32,
    pub avg_profit: f64,
    pub avg_profit_percent: f64,
    pub avg_gain: f64,
    pub avg_gain_percent: f64,
    pub avg_loss: f64,
    pub avg_loss_percent: f64,
    pub profitability_ratio: f64,
    pub profit_factor: f64,
    pub profit_and_loss_ratio: f64,
    pub total_fees: f64,
    pub net_profit: f64,
    pub beginning_equity: f64,
    pub ending_equity: f64,
    pub total_return: f64,
    pub daily_standard_deviation_percentage: f64,
    pub annual_standard_deviation_percentage: f64,
    pub max_drawdown_percentage_period: f64,
    pub max_drawdown_percentage_daily: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct PeriodTimeseriesStats {
    pub timestamp: i64,
    pub equity_value: BigDecimal,
    pub percent_drawdown: BigDecimal,
    pub cumulative_return: BigDecimal,
    pub period_return: BigDecimal,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct DailyTimeseriesStats {
    pub timestamp: i64,
    pub equity_value: BigDecimal,
    pub percent_drawdown: BigDecimal,
    pub cumulative_return: BigDecimal,
    pub period_return: BigDecimal,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Trades {
    pub trade_id: i32,
    pub leg_id: i32,
    pub timestamp: i64,
    pub ticker: String,
    pub quantity: BigDecimal,
    pub avg_price: BigDecimal,
    pub trade_value: BigDecimal,
    pub action: String,
    pub fees: BigDecimal,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Signals {
    pub timestamp: i64,
    pub trade_instructions: serde_json::Value,
}
