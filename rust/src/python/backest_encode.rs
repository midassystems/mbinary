use crate::backtest::BacktestData;
use crate::backtest_encode::BacktestEncoder;
use pyo3::prelude::*;

/// Python-facing wrapper for BacktestRecordEncoder
#[cfg_attr(feature = "python", pyclass(module = "mbinary"))]
pub struct PyBacktestEncoder {
    buffer: Vec<u8>, // Owned buffer
}

#[pymethods]
impl PyBacktestEncoder {
    /// Constructor for PyRecordEncoder
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyBacktestEncoder {
            buffer: Vec::new(), // Initialize with an empty buffer
        })
    }

    /// Encodes multiple records
    fn encode_backtest(&mut self, backtest: BacktestData) -> PyResult<Vec<u8>> {
        self.buffer.clear(); // Clear the buffer for new encoding

        let mut encoder = BacktestEncoder::new(&mut self.buffer);
        encoder.encode_metadata(&backtest.metadata);
        encoder.encode_timeseries(&backtest.period_timeseries_stats);
        encoder.encode_timeseries(&backtest.daily_timeseries_stats);
        encoder.encode_trades(&backtest.trades);
        encoder.encode_signals(&backtest.signals);

        Ok(self.buffer.clone())
    }
}
