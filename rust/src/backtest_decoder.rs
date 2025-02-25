use crate::backtest::BacktestMetaData;
use crate::backtest::Decode;
use crate::backtest::Signals;
use crate::backtest::TimeseriesStats;
use crate::backtest::Trades;
use crate::{Error, Result};
use std::io::Read;

/// Helper function to decode a vector with length prepended
fn decode_vector<T, R>(reader: &mut R) -> Result<Vec<T>>
where
    R: Read,      // Now works with anything implementing Read
    T: Decode<R>, // T must implement Decode<R>
{
    // Read the vector length (u32)
    let mut length_buf = [0u8; 4];
    reader
        .read_exact(&mut length_buf)
        .map_err(|_| Error::CustomError("Failed to read vector length".to_string()))?;
    let length = u32::from_le_bytes(length_buf) as usize;

    // Decode each element in the vector
    let mut result = Vec::with_capacity(length);
    for _ in 0..length {
        result.push(T::decode(reader)?);
    }

    Ok(result)
}

pub struct BacktestDecoder<R: Read> {
    cursor: R,
}

impl<R: Read> BacktestDecoder<R> {
    pub fn new(reader: R) -> Self {
        BacktestDecoder { cursor: reader }
    }

    pub fn decode_metadata(&mut self) -> Result<BacktestMetaData> {
        BacktestMetaData::decode(&mut self.cursor)
    }

    pub fn decode_timeseries(&mut self) -> Result<Vec<TimeseriesStats>> {
        decode_vector(&mut self.cursor)
    }

    pub fn decode_trades(&mut self) -> Result<Vec<Trades>> {
        decode_vector(&mut self.cursor)
    }

    pub fn decode_signals(&mut self) -> Result<Vec<Signals>> {
        decode_vector(&mut self.cursor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        backtest::{BacktestData, Parameters, SignalInstructions, StaticStats},
        backtest_encode::BacktestEncoder,
    };

    #[test]
    fn backtestencoder() -> anyhow::Result<()> {
        let params = Parameters {
            strategy_name: "Testing".to_string(),
            capital: 10000,
            schema: "Ohlcv-1s".to_string(),
            data_type: "BAR".to_string(),
            start: 1730160814000000000,
            end: 1730160814000000000,
            tickers: vec!["HE.n.0".to_string(), "AAPL".to_string()],
        };

        let static_stats = StaticStats {
            total_trades: 100,
            total_winning_trades: 50,
            total_losing_trades: 50,
            avg_profit: 1000000000000,
            avg_profit_percent: 10383783337737,
            avg_gain: 23323212233,
            avg_gain_percent: 24323234,
            avg_loss: 203982828,
            avg_loss_percent: 23432134323,
            profitability_ratio: 130213212323,
            profit_factor: 12342123431,
            profit_and_loss_ratio: 1234321343,
            total_fees: 123453234,
            net_profit: 1234323,
            beginning_equity: 12343234323,
            ending_equity: 12343234,
            total_return: 234532345,
            annualized_return: 234532345,
            daily_standard_deviation_percentage: 23453234,
            annual_standard_deviation_percentage: 34543443,
            max_drawdown_percentage_period: 234543234,
            max_drawdown_percentage_daily: 23432345,
            sharpe_ratio: 23432343,
            sortino_ratio: 123453234543,
        };
        let bt_metadata = BacktestMetaData::new(None, "testing", params, static_stats);

        let timeseries = TimeseriesStats {
            timestamp: 123700000000000,
            equity_value: 9999999,
            percent_drawdown: 2343234,
            cumulative_return: 2343234,
            period_return: 2345432345,
        };

        let stats: Vec<TimeseriesStats> = vec![timeseries.clone(), timeseries.clone()];

        let trade = Trades {
            trade_id: 1,
            signal_id: 1,
            timestamp: 1704903000,
            ticker: "AAPL".to_string(),
            quantity: 4,
            avg_price: 13074,
            trade_value: -52296,
            trade_cost: -52296,
            action: "BUY".to_string(),
            fees: 100,
        };

        let trades: Vec<Trades> = vec![trade.clone(), trade.clone()];

        let instructions = SignalInstructions {
            ticker: "AAPL".to_string(),
            order_type: "MKT".to_string(),
            action: "BUY".to_string(),
            signal_id: 2,
            weight: 13213432,
            quantity: 2343,
            limit_price: "12341".to_string(),
            aux_price: "1233212".to_string(),
        };

        let vec: Vec<SignalInstructions> = vec![instructions.clone(), instructions.clone()];
        let signal = Signals {
            timestamp: 1234565432345,
            trade_instructions: vec,
        };

        let signals: Vec<Signals> = vec![signal.clone(), signal.clone()];
        let backtest = BacktestData {
            metadata: bt_metadata,
            daily_timeseries_stats: stats.clone(),
            period_timeseries_stats: stats,
            trades,
            signals,
        };

        // Encode
        let mut bytes = Vec::new();
        let mut encoder = BacktestEncoder::new(&mut bytes);
        encoder.encode_metadata(&backtest.metadata);
        encoder.encode_timeseries(&backtest.period_timeseries_stats);
        encoder.encode_timeseries(&backtest.daily_timeseries_stats);
        encoder.encode_trades(&backtest.trades);
        encoder.encode_signals(&backtest.signals);

        // Decode
        let decode = bytes.as_slice();
        let mut decoder = BacktestDecoder::new(decode);

        let metadata = decoder.decode_metadata()?;
        let period_stats = decoder.decode_timeseries()?;
        let daily_stats = decoder.decode_timeseries()?;
        let trades = decoder.decode_trades()?;
        let signals = decoder.decode_signals()?;

        let decoded_backtest = BacktestData {
            metadata,
            period_timeseries_stats: period_stats,
            daily_timeseries_stats: daily_stats,
            trades,
            signals,
        };

        // Validate
        assert_eq!(backtest, decoded_backtest);

        Ok(())
    }
}
