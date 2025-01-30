use crate::backtest::{Parameters, Signals, Trades};
use crate::live::{AccountSummary, LiveData};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pymethods]
impl LiveData {
    #[new]
    #[pyo3(signature = (live_id=None,parameters= None, trades=None, signals=None, account=None))]
    fn py_new(
        live_id: Option<u16>,
        parameters: Option<Parameters>,
        trades: Option<Vec<Trades>>,
        signals: Option<Vec<Signals>>,
        account: Option<AccountSummary>,
    ) -> PyResult<Self> {
        Ok(LiveData {
            live_id,
            parameters: parameters.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'parameters' are required")
            })?,
            trades: trades.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'trades' are required")
            })?,
            signals: signals.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'signals' are required")
            })?,
            account: account.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'account' is required")
            })?,
        })
    }

    fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("live_id", self.live_id).unwrap();
        let _ = dict.set_item("parameters", self.parameters.to_dict(py));
        let _ = dict.set_item("account", self.account.to_dict(py));

        // Create a Python list to hold the trade instructions
        let trades_list = PyList::empty(py);
        for stat in &self.trades {
            let dict = stat.to_dict(py);
            trades_list.append(dict).unwrap();
        }

        let _ = dict.set_item("trades", &trades_list);

        // Create a Python list to hold the trade instructions
        let signal_list = PyList::empty(py);
        for stat in &self.signals {
            let dict = stat.to_dict(py);
            signal_list.append(dict).unwrap();
        }
        let _ = dict.set_item("signals", &signal_list);

        dict.into()
    }
}

#[pymethods]
impl AccountSummary {
    #[new]
    pub fn py_new(
        currency: String,
        start_timestamp: i64,
        start_buying_power: i64,
        start_excess_liquidity: i64,
        start_full_available_funds: i64,
        start_full_init_margin_req: i64,
        start_full_maint_margin_req: i64,
        start_futures_pnl: i64,
        start_net_liquidation: i64,
        start_total_cash_balance: i64,
        start_unrealized_pnl: i64,
        end_timestamp: i64,
        end_buying_power: i64,
        end_excess_liquidity: i64,
        end_full_available_funds: i64,
        end_full_init_margin_req: i64,
        end_full_maint_margin_req: i64,
        end_futures_pnl: i64,
        end_net_liquidation: i64,
        end_total_cash_balance: i64,
        end_unrealized_pnl: i64,
    ) -> Self {
        AccountSummary {
            currency,
            start_timestamp,
            start_buying_power,
            start_excess_liquidity,
            start_full_available_funds,
            start_full_init_margin_req,
            start_full_maint_margin_req,
            start_futures_pnl,
            start_net_liquidation,
            start_total_cash_balance,
            start_unrealized_pnl,
            end_timestamp,
            end_buying_power,
            end_excess_liquidity,
            end_full_available_funds,
            end_full_init_margin_req,
            end_full_maint_margin_req,
            end_futures_pnl,
            end_net_liquidation,
            end_total_cash_balance,
            end_unrealized_pnl,
        }
    }
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("currency", &self.currency).unwrap();
        dict.set_item("start_timestamp", &self.start_timestamp)
            .unwrap();
        dict.set_item("start_buying_power", &self.start_buying_power)
            .unwrap();
        dict.set_item("start_excess_liquidity", &self.start_excess_liquidity)
            .unwrap();
        dict.set_item(
            "start_full_available_funds",
            &self.start_full_available_funds,
        )
        .unwrap();
        dict.set_item(
            "start_full_init_margin_req",
            &self.start_full_init_margin_req,
        )
        .unwrap();
        dict.set_item(
            "start_full_maint_margin_req",
            &self.start_full_maint_margin_req,
        )
        .unwrap();
        dict.set_item("start_futures_pnl", &self.start_futures_pnl)
            .unwrap();
        dict.set_item("start_net_liquidation", &self.start_net_liquidation)
            .unwrap();
        dict.set_item("start_total_cash_balance", &self.start_total_cash_balance)
            .unwrap();
        dict.set_item("start_unrealized_pnl", &self.start_unrealized_pnl)
            .unwrap();
        dict.set_item("end_timestamp", &self.end_timestamp).unwrap();
        dict.set_item("end_buying_power", &self.end_buying_power)
            .unwrap();
        dict.set_item("end_excess_liquidity", &self.end_excess_liquidity)
            .unwrap();
        dict.set_item("end_full_available_funds", &self.end_full_available_funds)
            .unwrap();
        dict.set_item("end_full_init_margin_req", &self.end_full_init_margin_req)
            .unwrap();
        dict.set_item("end_full_maint_margin_req", &self.end_full_maint_margin_req)
            .unwrap();
        dict.set_item("end_futures_pnl", &self.end_futures_pnl)
            .unwrap();
        dict.set_item("end_net_liquidation", &self.end_net_liquidation)
            .unwrap();
        dict.set_item("end_total_cash_balance", &self.end_total_cash_balance)
            .unwrap();
        dict.set_item("end_unrealized_pnl", &self.end_unrealized_pnl)
            .unwrap();

        dict.into()
    }
}
