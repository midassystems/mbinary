use crate::backtest::{
    BacktestData, Parameters, SignalInstructions, Signals, StaticStats, TimeseriesStats, Trades,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pymethods]
impl BacktestData {
    #[new]
    #[pyo3(signature = (backtest_id = None, backtest_name=None, parameters=None, static_stats=None, period_timeseries_stats=None, daily_timeseries_stats=None, trades=None, signals=None))]
    pub fn py_new(
        backtest_id: Option<u16>,
        backtest_name: Option<String>,
        parameters: Option<Parameters>,
        static_stats: Option<StaticStats>,
        period_timeseries_stats: Option<Vec<TimeseriesStats>>,
        daily_timeseries_stats: Option<Vec<TimeseriesStats>>,
        trades: Option<Vec<Trades>>,
        signals: Option<Vec<Signals>>,
    ) -> PyResult<Self> {
        Ok(BacktestData {
            backtest_id,
            backtest_name: backtest_name.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'backtest_name' is required")
            })?,
            parameters: parameters.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'parameters' are required")
            })?,
            static_stats: static_stats.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'static_stats' are required")
            })?,
            period_timeseries_stats: period_timeseries_stats.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "'period_timeseries_stats' are required",
                )
            })?,
            daily_timeseries_stats: daily_timeseries_stats.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "'daily_timeseries_stats' are required",
                )
            })?,
            trades: trades.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'trades' are required")
            })?,
            signals: signals.clone().ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("'signals' are required")
            })?,
        })
    }
    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("backtest_id", self.backtest_id).unwrap();
        dict.set_item("backtest_name", &self.backtest_name).unwrap();
        let _ = dict.set_item("parameters", self.parameters.__dict__(py));
        let _ = dict.set_item("static_stats", self.static_stats.__dict__(py));

        // Create a Python list to hold the trade instructions
        let period_list = PyList::empty_bound(py);
        for stat in &self.period_timeseries_stats {
            let dict = stat.__dict__(py);
            period_list.append(dict).unwrap();
        }
        let _ = dict.set_item("period_timeseries_stats", &period_list);

        // Create a Python list to hold the trade instructions
        let daily_list = PyList::empty_bound(py);
        for stat in &self.daily_timeseries_stats {
            let dict = stat.__dict__(py);
            daily_list.append(dict).unwrap();
        }

        let _ = dict.set_item("daily_timeseries_stats", &daily_list);

        // Create a Python list to hold the trade instructions
        let trades_list = PyList::empty_bound(py);
        for stat in &self.trades {
            let dict = stat.__dict__(py);
            trades_list.append(dict).unwrap();
        }

        let _ = dict.set_item("trades", &trades_list);

        // Create a Python list to hold the trade instructions
        let signal_list = PyList::empty_bound(py);
        for stat in &self.signals {
            let dict = stat.__dict__(py);
            signal_list.append(dict).unwrap();
        }
        let _ = dict.set_item("signals", &signal_list);

        dict.into()
    }
}

#[pymethods]
impl Parameters {
    #[new]
    pub fn py_new(
        strategy_name: String,
        capital: i64,
        schema: String,
        data_type: String,
        train_start: i64,
        train_end: i64,
        test_start: i64,
        test_end: i64,
        tickers: Vec<String>,
    ) -> Self {
        Parameters {
            strategy_name,
            capital,
            schema,
            data_type,
            train_start,
            train_end,
            test_start,
            test_end,
            tickers,
        }
    }
    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("strategy_name", &self.strategy_name).unwrap();
        dict.set_item("capital", self.capital).unwrap();
        dict.set_item("schema", &self.schema).unwrap();
        dict.set_item("data_type", &self.data_type).unwrap();
        dict.set_item("train_start", self.train_start).unwrap();
        dict.set_item("train_end", self.train_end).unwrap();
        dict.set_item("test_start", self.test_start).unwrap();
        dict.set_item("test_end", self.test_end).unwrap();
        dict.set_item("tickers", &self.tickers).unwrap();
        dict.into()
    }
}
#[pymethods]
impl StaticStats {
    #[new]
    pub fn py_new(
        total_trades: i32,
        total_winning_trades: i32,
        total_losing_trades: i32,
        avg_profit: i64,
        avg_profit_percent: i64,
        avg_gain: i64,
        avg_gain_percent: i64,
        avg_loss: i64,
        avg_loss_percent: i64,
        profitability_ratio: i64,
        profit_factor: i64,
        profit_and_loss_ratio: i64,
        total_fees: i64,
        net_profit: i64,
        beginning_equity: i64,
        ending_equity: i64,
        total_return: i64,
        daily_standard_deviation_percentage: i64,
        annual_standard_deviation_percentage: i64,
        max_drawdown_percentage_period: i64,
        max_drawdown_percentage_daily: i64,
        sharpe_ratio: i64,
        sortino_ratio: i64,
    ) -> Self {
        StaticStats {
            total_trades,
            total_winning_trades,
            total_losing_trades,
            avg_profit,
            avg_profit_percent,
            avg_gain,
            avg_gain_percent,
            avg_loss,
            avg_loss_percent,
            profitability_ratio,
            profit_factor,
            profit_and_loss_ratio,
            total_fees,
            net_profit,
            beginning_equity,
            ending_equity,
            total_return,
            daily_standard_deviation_percentage,
            annual_standard_deviation_percentage,
            max_drawdown_percentage_period,
            max_drawdown_percentage_daily,
            sharpe_ratio,
            sortino_ratio,
        }
    }

    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("total_trades", &self.total_trades).unwrap();
        dict.set_item("total_winning_trades", self.total_winning_trades)
            .unwrap();
        dict.set_item("total_losing_trades", &self.total_losing_trades)
            .unwrap();
        dict.set_item("avg_profit", &self.avg_profit).unwrap();
        dict.set_item("avg_profit_percent", self.avg_profit_percent)
            .unwrap();
        dict.set_item("avg_gain", self.avg_gain).unwrap();
        dict.set_item("avg_gain_percent", self.avg_gain_percent)
            .unwrap();
        dict.set_item("avg_loss", self.avg_loss).unwrap();
        dict.set_item("avg_loss_percent", self.avg_loss_percent)
            .unwrap();
        dict.set_item("profitability_ratio", self.profitability_ratio)
            .unwrap();
        dict.set_item("profit_factor", &self.profit_factor).unwrap();
        dict.set_item("profit_and_loss_ratio", &self.profit_and_loss_ratio)
            .unwrap();
        dict.set_item("total_fees", &self.total_fees).unwrap();
        dict.set_item("net_profit", &self.net_profit).unwrap();
        dict.set_item("beginning_equity", &self.beginning_equity)
            .unwrap();
        dict.set_item("ending_equity", &self.ending_equity).unwrap();
        dict.set_item("total_return", &self.total_return).unwrap();
        dict.set_item(
            "daily_standard_deviation_percentage",
            &self.daily_standard_deviation_percentage,
        )
        .unwrap();
        dict.set_item(
            "annual_standard_deviation_percentage",
            &self.annual_standard_deviation_percentage,
        )
        .unwrap();
        dict.set_item(
            "max_drawdown_percentage_daily",
            &self.max_drawdown_percentage_daily,
        )
        .unwrap();
        dict.set_item(
            "max_drawdown_percentage_period",
            &self.max_drawdown_percentage_period,
        )
        .unwrap();
        dict.set_item("sharpe_ratio", &self.sharpe_ratio).unwrap();
        dict.set_item("sortino_ratio", &self.sortino_ratio).unwrap();

        dict.into()
    }
}

#[pymethods]
impl TimeseriesStats {
    #[new]
    pub fn py_new(
        timestamp: i64,
        equity_value: i64,
        percent_drawdown: i64,
        cumulative_return: i64,
        period_return: i64,
    ) -> Self {
        TimeseriesStats {
            timestamp,
            equity_value,
            percent_drawdown,
            cumulative_return,
            period_return,
        }
    }

    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("timestamp", &self.timestamp).unwrap();
        dict.set_item("equity_value", self.equity_value).unwrap();
        dict.set_item("percent_drawdown", &self.percent_drawdown)
            .unwrap();
        dict.set_item("period_return", &self.period_return).unwrap();
        dict.set_item("cumulative_return", &self.cumulative_return)
            .unwrap();

        dict.into()
    }
}

#[pymethods]
impl Trades {
    #[new]
    pub fn py_new(
        trade_id: i32,
        leg_id: i32,
        timestamp: i64,
        ticker: String,
        quantity: i64,
        avg_price: i64,
        trade_value: i64,
        action: String,
        fees: i64,
    ) -> Self {
        Trades {
            trade_id,
            leg_id,
            timestamp,
            ticker,
            quantity,
            avg_price,
            trade_value,
            action,
            fees,
        }
    }
    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("trade_id", self.trade_id).unwrap();
        dict.set_item("leg_id", self.leg_id).unwrap();
        dict.set_item("timestamp", self.timestamp).unwrap();
        dict.set_item("ticker", &self.ticker).unwrap();
        dict.set_item("quantity", self.quantity).unwrap();
        dict.set_item("avg_price", self.avg_price).unwrap();
        dict.set_item("trade_value", self.trade_value).unwrap();
        dict.set_item("action", &self.action).unwrap();
        dict.set_item("fees", self.fees).unwrap();
        dict.into()
    }
}

#[pymethods]
impl Signals {
    #[new]
    pub fn py_new(timestamp: i64, trade_instructions: Vec<SignalInstructions>) -> Self {
        Signals {
            timestamp,
            trade_instructions,
        }
    }
    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("timestamp", &self.timestamp).unwrap();

        // Create a Python list to hold the trade instructions
        let trade_instructions_list = PyList::empty_bound(py);

        // Iterate over the trade_instructions vector
        for instruction in &self.trade_instructions {
            let instruction_dict = instruction.__dict__(py);
            trade_instructions_list.append(instruction_dict).unwrap();
        }
        let _ = dict.set_item("trade_instructions", &trade_instructions_list);

        dict.into()
    }
}

#[pymethods]
impl SignalInstructions {
    #[new]
    pub fn py_new(
        ticker: String,
        order_type: String,
        action: String,
        trade_id: i32,
        leg_id: i32,
        weight: i64,
        quantity: i32,
        limit_price: String,
        aux_price: String,
    ) -> Self {
        SignalInstructions {
            ticker,
            order_type,
            action,
            trade_id,
            leg_id,
            weight,
            quantity,
            limit_price,
            aux_price,
        }
    }
    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("ticker", &self.ticker).unwrap();
        dict.set_item("order_type", &self.order_type).unwrap();
        dict.set_item("action", &self.action).unwrap();
        dict.set_item("trade_id", self.trade_id).unwrap();
        dict.set_item("leg_id", self.leg_id).unwrap();
        dict.set_item("weight", self.weight).unwrap();
        dict.set_item("quantity", self.quantity).unwrap();
        dict.set_item("limit_price", &self.limit_price).unwrap();
        dict.set_item("aux_price", &self.aux_price).unwrap();
        dict.into()
    }
}
