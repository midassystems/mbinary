use crate::backtest::{Parameters, Signals, Trades};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[cfg(feature = "python")]
use pyo3::pyclass;

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct LiveData {
    pub live_id: Option<u16>,
    pub parameters: Parameters,
    pub trades: Vec<Trades>,
    pub signals: Vec<Signals>,
    pub account: AccountSummary,
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct AccountSummary {
    pub currency: String,
    pub start_timestamp: i64,
    pub start_buying_power: i64,
    pub start_excess_liquidity: i64,
    pub start_full_available_funds: i64,
    pub start_full_init_margin_req: i64,
    pub start_full_maint_margin_req: i64,
    pub start_futures_pnl: i64,
    pub start_net_liquidation: i64,
    pub start_total_cash_balance: i64,
    pub start_unrealized_pnl: i64,
    pub end_timestamp: i64,
    pub end_buying_power: i64,
    pub end_excess_liquidity: i64,
    pub end_full_available_funds: i64,
    pub end_full_init_margin_req: i64,
    pub end_full_maint_margin_req: i64,
    pub end_futures_pnl: i64,
    pub end_net_liquidation: i64,
    pub end_total_cash_balance: i64,
    pub end_unrealized_pnl: i64,
}
