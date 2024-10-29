use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[cfg(feature = "python")]
use pyo3::pyclass;

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct BacktestData {
    pub backtest_id: Option<u16>,
    pub backtest_name: String,
    pub parameters: Parameters,
    pub static_stats: StaticStats,
    pub period_timeseries_stats: Vec<TimeseriesStats>,
    pub daily_timeseries_stats: Vec<TimeseriesStats>,
    pub trades: Vec<Trades>,
    pub signals: Vec<Signals>,
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct Parameters {
    pub strategy_name: String,
    pub capital: i64,
    pub schema: String,
    pub data_type: String,
    pub train_start: i64,
    pub train_end: i64,
    pub test_start: i64,
    pub test_end: i64,
    pub tickers: Vec<String>,
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct StaticStats {
    pub total_trades: i32,
    pub total_winning_trades: i32,
    pub total_losing_trades: i32,
    pub avg_profit: i64,                           // Scaled by 1e9
    pub avg_profit_percent: i64,                   // Scaled by 1e9
    pub avg_gain: i64,                             // Scaled by 1e9
    pub avg_gain_percent: i64,                     // Scaled by 1e9
    pub avg_loss: i64,                             // Scaled by 1e9
    pub avg_loss_percent: i64,                     // Scaled by 1e9
    pub profitability_ratio: i64,                  // Scaled by 1e9
    pub profit_factor: i64,                        // Scaled by 1e9
    pub profit_and_loss_ratio: i64,                // Scaled by 1e9
    pub total_fees: i64,                           // Scaled by 1e9
    pub net_profit: i64,                           // Scaled by 1e9
    pub beginning_equity: i64,                     // Scaled by 1e9
    pub ending_equity: i64,                        // Scaled by 1e9
    pub total_return: i64,                         // Scaled by 1e9
    pub daily_standard_deviation_percentage: i64,  // Scaled by 1e9
    pub annual_standard_deviation_percentage: i64, // Scaled by 1e9
    pub max_drawdown_percentage_period: i64,       // Scaled by 1e9
    pub max_drawdown_percentage_daily: i64,        // Scaled by 1e9
    pub sharpe_ratio: i64,                         // Scaled by 1e9
    pub sortino_ratio: i64,                        // Scaled by 1e9
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct TimeseriesStats {
    pub timestamp: i64,
    pub equity_value: i64,      // Scaled by 1e9
    pub percent_drawdown: i64,  // Scaled by 1e9
    pub cumulative_return: i64, // Scaled by 1e9
    pub period_return: i64,     // Scaled by 1e9
}

// #[repr(C)]
// #[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
// #[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
// pub struct DailyTimeseriesStats {
//     pub timestamp: i64,
//     pub equity_value: i64,      // Scaled by 1e9
//     pub percent_drawdown: i64,  // Scaled by 1e9
//     pub cumulative_return: i64, // Scaled by 1e9
//     pub period_return: i64,     // Scaled by 1e9
// }

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Trades {
    pub trade_id: i32,
    pub leg_id: i32,
    pub timestamp: i64,
    pub ticker: String,
    pub quantity: i64,    // Scaled by 1e9
    pub avg_price: i64,   // Scaled by 1e9
    pub trade_value: i64, // Scaled by 1e9
    pub action: String,
    pub fees: i64, // Scaled by 1e9
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Signals {
    pub timestamp: i64,
    pub trade_instructions: Vec<SignalInstructions>,
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct SignalInstructions {
    pub ticker: String,
    pub order_type: String,
    pub action: String,
    pub trade_id: i32,
    pub leg_id: i32,
    pub weight: i64, // Scaled by 1e9
    pub quantity: i32,
    pub limit_price: String, // Maybe int scale by 1e9
    pub aux_price: String,   // Myabe int scale by 1e9
}
