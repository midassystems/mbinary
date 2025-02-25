use crate::backtest::{
    BacktestData, BacktestMetaData, Parameters, SignalInstructions, Signals, StaticStats,
    TimeseriesStats, Trades,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pymethods]
impl BacktestData {
    #[new]
    pub fn py_new(
        metadata: BacktestMetaData,
        period_timeseries_stats: Vec<TimeseriesStats>,
        daily_timeseries_stats: Vec<TimeseriesStats>,
        trades: Vec<Trades>,
        signals: Vec<Signals>,
    ) -> PyResult<Self> {
        Ok(BacktestData {
            metadata,
            period_timeseries_stats,
            daily_timeseries_stats,
            trades,
            signals,
        })
    }
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        let _ = dict.set_item("metadata", self.metadata.__dict__(py));

        // Create a Python list to hold the trade instructions
        let period_list = PyList::empty(py);
        for stat in &self.period_timeseries_stats {
            let dict = stat.to_dict(py);
            period_list.append(dict).unwrap();
        }
        let _ = dict.set_item("period_timeseries_stats", &period_list);

        // Create a Python list to hold the trade instructions
        let daily_list = PyList::empty(py);
        for stat in &self.daily_timeseries_stats {
            let dict = stat.to_dict(py);
            daily_list.append(dict).unwrap();
        }

        let _ = dict.set_item("daily_timeseries_stats", &daily_list);

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
impl BacktestMetaData {
    #[new]
    pub fn py_new(
        backtest_id: u16,
        backtest_name: String,
        parameters: Parameters,
        static_stats: StaticStats,
    ) -> PyResult<Self> {
        Ok(BacktestMetaData {
            backtest_id,
            backtest_name,
            parameters,
            static_stats,
        })
    }

    pub fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("backtest_id", self.backtest_id).unwrap();
        dict.set_item("backtest_name", &self.backtest_name).unwrap();
        let _ = dict.set_item("parameters", self.parameters.to_dict(py));
        let _ = dict.set_item("static_stats", self.static_stats.to_dict(py));

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
        start: i64,
        end: i64,
        tickers: Vec<String>,
    ) -> Self {
        Parameters {
            strategy_name,
            capital,
            schema,
            data_type,
            start,
            end,
            tickers,
        }
    }
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("strategy_name", &self.strategy_name).unwrap();
        dict.set_item("capital", self.capital).unwrap();
        dict.set_item("schema", &self.schema).unwrap();
        dict.set_item("data_type", &self.data_type).unwrap();
        dict.set_item("start", self.start).unwrap();
        dict.set_item("end", self.end).unwrap();
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
        annualized_return: i64,
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
            annualized_return,
            daily_standard_deviation_percentage,
            annual_standard_deviation_percentage,
            max_drawdown_percentage_period,
            max_drawdown_percentage_daily,
            sharpe_ratio,
            sortino_ratio,
        }
    }

    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
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
        dict.set_item("annualized_return", &self.annualized_return)
            .unwrap();
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

    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
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
        signal_id: i32,
        timestamp: i64,
        ticker: String,
        quantity: i64,
        avg_price: i64,
        trade_value: i64,
        trade_cost: i64,
        action: String,
        fees: i64,
    ) -> Self {
        Trades {
            trade_id,
            signal_id,
            timestamp,
            ticker,
            quantity,
            avg_price,
            trade_value,
            trade_cost,
            action,
            fees,
        }
    }
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("trade_id", self.trade_id).unwrap();
        dict.set_item("signal_id", self.signal_id).unwrap();
        dict.set_item("timestamp", self.timestamp).unwrap();
        dict.set_item("ticker", &self.ticker).unwrap();
        dict.set_item("quantity", self.quantity).unwrap();
        dict.set_item("avg_price", self.avg_price).unwrap();
        dict.set_item("trade_value", self.trade_value).unwrap();
        dict.set_item("trade_cost", self.trade_cost).unwrap();
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
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("timestamp", &self.timestamp).unwrap();

        // Create a Python list to hold the trade instructions
        let trade_instructions_list = PyList::empty(py);

        // Iterate over the trade_instructions vector
        for instruction in &self.trade_instructions {
            let instruction_dict = instruction.to_dict(py);
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
        signal_id: i32,
        weight: i64,
        quantity: i32,
        limit_price: String,
        aux_price: String,
    ) -> Self {
        SignalInstructions {
            ticker,
            order_type,
            action,
            signal_id,
            weight,
            quantity,
            limit_price,
            aux_price,
        }
    }
    pub fn to_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("ticker", &self.ticker).unwrap();
        dict.set_item("order_type", &self.order_type).unwrap();
        dict.set_item("action", &self.action).unwrap();
        dict.set_item("signal_id", self.signal_id).unwrap();
        dict.set_item("weight", self.weight).unwrap();
        dict.set_item("quantity", self.quantity).unwrap();
        dict.set_item("limit_price", &self.limit_price).unwrap();
        dict.set_item("aux_price", &self.aux_price).unwrap();
        dict.into()
    }
}
