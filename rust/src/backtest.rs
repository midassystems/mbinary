use crate::{Error, Result};
use bytemuck;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{io::Read, u16};

#[cfg(feature = "python")]
use pyo3::pyclass;

/// Helper to write a string as 2-byte length-prefixed UTF-8
fn write_string(buffer: &mut Vec<u8>, string: &str) {
    let length = string.len() as u16; // Convert length to u16
    buffer.extend(&length.to_le_bytes()); // Write the 2-byte length
    buffer.extend(string.as_bytes()); // Write the UTF-8 bytes
}

/// Helper to read a 2-byte length-prefixed UTF-8 string
fn read_string<R: Read>(cursor: &mut R) -> Result<String> {
    let mut len_buf = [0u8; 2]; // Buffer to store the 2-byte length
    cursor
        .read_exact(&mut len_buf)
        .map_err(|_| Error::CustomError("Failed to read string length".to_string()))?;
    let len = u16::from_le_bytes(len_buf) as usize; // Convert length to usize

    let mut string_buf = vec![0u8; len]; // Buffer to store the string bytes
    cursor
        .read_exact(&mut string_buf)
        .map_err(|_| Error::CustomError("Failed to read string bytes".to_string()))?;
    String::from_utf8(string_buf)
        .map_err(|_| Error::CustomError("Invalid UTF-8 string".to_string()))
}

/// Helper function to read fixed-size data (e.g., i32, i64) from the cursor.
fn read_fixed<T: Sized + Copy, R: Read>(cursor: &mut R) -> Result<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    let mut buffer = vec![0u8; std::mem::size_of::<T>()];
    cursor.read_exact(&mut buffer).map_err(|_| {
        Error::CustomError(format!("Failed to read {} bytes", std::mem::size_of::<T>()))
    })?;
    Ok(bytemuck::cast_slice(&buffer)[0])
}

/// Trait to define encoding for individual items
pub trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>);
}

pub trait Decode<R: Read>: Sized {
    fn decode(cursor: &mut R) -> Result<Self>;
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct BacktestData {
    pub metadata: BacktestMetaData,
    pub period_timeseries_stats: Vec<TimeseriesStats>,
    pub daily_timeseries_stats: Vec<TimeseriesStats>,
    pub trades: Vec<Trades>,
    pub signals: Vec<Signals>,
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct BacktestMetaData {
    pub backtest_id: u16,
    pub backtest_name: String,
    pub parameters: Parameters,
    pub static_stats: StaticStats,
}

impl BacktestMetaData {
    pub fn new(
        mut backtest_id: Option<u16>,
        backtest_name: &str,
        parameters: Parameters,
        static_stats: StaticStats,
    ) -> Self {
        if None == backtest_id {
            backtest_id = Some(u16::MAX); // Sentinel value
        };
        Self {
            backtest_id: backtest_id.unwrap(),
            backtest_name: backtest_name.to_string(),
            parameters,
            static_stats,
        }
    }
}

impl Encode for BacktestMetaData {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&self.backtest_id.to_le_bytes());
        write_string(buffer, &self.backtest_name);
        self.parameters.encode(buffer);
        self.static_stats.encode(buffer);
    }
}

impl<R: Read> Decode<R> for BacktestMetaData {
    fn decode(cursor: &mut R) -> Result<Self> {
        let backtest_id: u16 = read_fixed(cursor)?;
        let backtest_name: String = read_string(cursor)?;
        let parameters: Parameters = Parameters::decode(cursor)?;
        let static_stats: StaticStats = StaticStats::decode(cursor)?;

        Ok(Self {
            backtest_id,
            backtest_name,
            parameters,
            static_stats,
        })
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq)]
pub struct Parameters {
    pub strategy_name: String,
    pub capital: i64,
    pub schema: String,
    pub data_type: String,
    pub start: i64,
    pub end: i64,
    pub tickers: Vec<String>,
}

impl Encode for Parameters {
    fn encode(&self, buffer: &mut Vec<u8>) {
        write_string(buffer, &self.strategy_name);
        buffer.extend(&self.capital.to_le_bytes());
        write_string(buffer, &self.schema);
        write_string(buffer, &self.data_type);
        buffer.extend(&self.start.to_le_bytes());
        buffer.extend(&self.end.to_le_bytes());

        // Encode tickers length as prefix to list
        let tickers_len = self.tickers.len() as u32;
        buffer.extend(&tickers_len.to_le_bytes());

        for ticker in &self.tickers {
            write_string(buffer, ticker);
        }
    }
}

impl<R: Read> Decode<R> for Parameters {
    fn decode(cursor: &mut R) -> Result<Self> {
        // fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        // let mut cursor = std::io::Cursor::new(buffer);

        let strategy_name: String = read_string(cursor)?;
        let capital: i64 = read_fixed(cursor)?;
        let schema: String = read_string(cursor)?;
        let data_type: String = read_string(cursor)?;
        let start: i64 = read_fixed(cursor)?;
        let end: i64 = read_fixed(cursor)?;

        // Decode tickers length, 4 b/c stored as u32(4 bytes)
        let mut tickers_len_buf = [0u8; 4];
        cursor
            .read_exact(&mut tickers_len_buf)
            .map_err(|_| Error::CustomError("Failed to read tickers length".to_string()))?;
        let tickers_len = u32::from_le_bytes(tickers_len_buf) as usize;

        //Decode tickers
        let mut tickers = Vec::new();
        for _ in 0..tickers_len {
            tickers.push(read_string(cursor)?);
        }

        Ok(Self {
            strategy_name,
            capital,
            schema,
            data_type,
            start,
            end,
            tickers,
        })
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
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
    pub annualized_return: i64,                    // Scaled by 1e9
    pub daily_standard_deviation_percentage: i64,  // Scaled by 1e9
    pub annual_standard_deviation_percentage: i64, // Scaled by 1e9
    pub max_drawdown_percentage_period: i64,       // Scaled by 1e9
    pub max_drawdown_percentage_daily: i64,        // Scaled by 1e9
    pub sharpe_ratio: i64,                         // Scaled by 1e9
    pub sortino_ratio: i64,                        // Scaled by 1e9
}

impl Encode for StaticStats {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(unsafe {
            std::slice::from_raw_parts(
                (self as *const StaticStats) as *const u8,
                std::mem::size_of::<StaticStats>(),
            )
        });
    }
}

impl<R: Read> Decode<R> for StaticStats {
    fn decode(cursor: &mut R) -> Result<Self> {
        // Read the required bytes into a temporary buffer
        let mut buffer = vec![0u8; std::mem::size_of::<StaticStats>()];
        cursor.read_exact(&mut buffer)?;

        // Create a pointer to the buffer and cast it to a `StaticStats` pointer
        let ptr = buffer.as_ptr() as *const StaticStats;

        // Dereference the pointer to create a `StaticStats` instance
        unsafe { Ok(ptr.read()) }
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct TimeseriesStats {
    pub timestamp: i64,
    pub equity_value: i64,      // Scaled by 1e9
    pub percent_drawdown: i64,  // Scaled by 1e9
    pub cumulative_return: i64, // Scaled by 1e9
    pub period_return: i64,     // Scaled by 1e9
}

impl Encode for TimeseriesStats {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(unsafe {
            // Serialize the `TimeseriesStats` struct as a slice of bytes.
            std::slice::from_raw_parts(
                (self as *const TimeseriesStats) as *const u8,
                std::mem::size_of::<TimeseriesStats>(),
            )
        });
    }
}

impl<R: Read> Decode<R> for TimeseriesStats {
    fn decode(cursor: &mut R) -> Result<Self> {
        // Read the required bytes into a temporary buffer
        let mut buffer = vec![0u8; std::mem::size_of::<TimeseriesStats>()];
        cursor.read_exact(&mut buffer)?;

        // Create a pointer to the buffer and cast it to a `StaticStats` pointer
        let ptr = buffer.as_ptr() as *const TimeseriesStats;

        // Dereference the pointer to create a `StaticStats` instance
        unsafe { Ok(ptr.read()) }
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Trades {
    pub trade_id: i32,
    pub signal_id: i32,
    pub timestamp: i64,
    pub ticker: String,
    pub quantity: i64,    // Scaled by 1e9
    pub avg_price: i64,   // Scaled by 1e9
    pub trade_value: i64, // Scaled by 1e9
    pub trade_cost: i64,  // Scaled by 1e9
    pub action: String,
    pub fees: i64, // Scaled by 1e9
}

impl Encode for Trades {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&self.trade_id.to_le_bytes());
        buffer.extend(&self.signal_id.to_le_bytes());
        buffer.extend(&self.timestamp.to_le_bytes());
        write_string(buffer, &self.ticker);
        buffer.extend(&self.quantity.to_le_bytes());
        buffer.extend(&self.avg_price.to_le_bytes());
        buffer.extend(&self.trade_value.to_le_bytes());
        buffer.extend(&self.trade_cost.to_le_bytes());
        write_string(buffer, &self.action);
        buffer.extend(&self.fees.to_le_bytes());
    }
}

impl<R: Read> Decode<R> for Trades {
    fn decode(cursor: &mut R) -> Result<Self> {
        let trade_id: i32 = read_fixed(cursor)?;
        let signal_id: i32 = read_fixed(cursor)?;
        let timestamp: i64 = read_fixed(cursor)?;
        let ticker: String = read_string(cursor)?;
        let quantity: i64 = read_fixed(cursor)?;
        let avg_price: i64 = read_fixed(cursor)?;
        let trade_value: i64 = read_fixed(cursor)?;
        let trade_cost: i64 = read_fixed(cursor)?;
        let action: String = read_string(cursor)?;
        let fees: i64 = read_fixed(cursor)?;

        Ok(Self {
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
        })
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Signals {
    pub timestamp: i64,
    pub trade_instructions: Vec<SignalInstructions>,
}

impl Encode for Signals {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&self.timestamp.to_le_bytes());

        // Encode trade_instructions length as prefix to list
        let trade_len = self.trade_instructions.len() as u32;
        buffer.extend(&trade_len.to_le_bytes());

        for t in &self.trade_instructions {
            t.encode(buffer);
        }
    }
}

impl<R: Read> Decode<R> for Signals {
    fn decode(cursor: &mut R) -> Result<Self> {
        let timestamp: i64 = read_fixed(cursor)?;

        // Decode tickers length, 4 b/c stored as u32(4 bytes)
        let instruction_len: u32 = read_fixed(cursor)?;

        //Decode tickers
        let mut trade_instructions = Vec::new();
        for _ in 0..instruction_len {
            trade_instructions.push(SignalInstructions::decode(cursor)?);
        }

        Ok(Self {
            timestamp,
            trade_instructions,
        })
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Deserialize, Serialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct SignalInstructions {
    pub ticker: String,
    pub order_type: String,
    pub action: String,
    pub signal_id: i32,
    pub weight: i64, // Scaled by 1e9
    pub quantity: i32,
    pub limit_price: String, // Maybe int scale by 1e9
    pub aux_price: String,   // Myabe int scale by 1e9
}

impl Encode for SignalInstructions {
    fn encode(&self, buffer: &mut Vec<u8>) {
        write_string(buffer, &self.ticker);
        write_string(buffer, &self.order_type);
        write_string(buffer, &self.action);
        buffer.extend(&self.signal_id.to_le_bytes());
        buffer.extend(&self.weight.to_le_bytes());
        buffer.extend(&self.quantity.to_le_bytes());
        write_string(buffer, &self.limit_price);
        write_string(buffer, &self.aux_price);
    }
}
impl<R: Read> Decode<R> for SignalInstructions {
    fn decode(cursor: &mut R) -> Result<Self> {
        let ticker: String = read_string(cursor)?;
        let order_type: String = read_string(cursor)?;
        let action: String = read_string(cursor)?;
        let signal_id: i32 = read_fixed(cursor)?;
        let weight: i64 = read_fixed(cursor)?;
        let quantity: i32 = read_fixed(cursor)?;
        let limit_price: String = read_string(cursor)?;
        let aux_price: String = read_string(cursor)?;

        Ok(Self {
            ticker,
            order_type,
            action,
            signal_id,
            weight,
            quantity,
            limit_price,
            aux_price,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_encode_valid_string() -> anyhow::Result<()> {
        // Encode String
        let text = "testing123";
        let mut buffer = Vec::new();
        write_string(&mut buffer, &text);

        // Decode String
        let bytes: &[u8] = &buffer;
        let mut cursor = Cursor::new(bytes);
        let decoded = read_string(&mut cursor)?;

        //Validate
        assert_eq!(text, &decoded);

        Ok(())
    }

    #[test]
    fn test_encode_empty_string() -> anyhow::Result<()> {
        // Encode String
        let text = "";
        let mut buffer = Vec::new();
        write_string(&mut buffer, &text);

        // Decode String
        let bytes: &[u8] = &buffer;
        let mut cursor = Cursor::new(bytes);
        let decoded = read_string(&mut cursor)?;

        //Validate
        assert_eq!(text, &decoded);

        Ok(())
    }

    #[test]
    fn parameters_encode_decode() -> anyhow::Result<()> {
        let params = Parameters {
            strategy_name: "Testing".to_string(),
            capital: 10000,
            schema: "Ohlcv-1s".to_string(),
            data_type: "BAR".to_string(),
            start: 1730160814000000000,
            end: 1730160814000000000,
            tickers: vec!["HE.n.0".to_string(), "AAPL".to_string()],
        };

        // Encode
        let mut bytes = Vec::new();
        params.encode(&mut bytes);

        // Decode
        let mut cursor = Cursor::new(bytes.as_slice());
        let decoded = Parameters::decode(&mut cursor)?;

        // Validate
        assert_eq!(params, decoded);

        Ok(())
    }

    #[test]
    fn staticstats_encode_decode() -> anyhow::Result<()> {
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

        // Encode
        let mut bytes = Vec::new();
        static_stats.encode(&mut bytes);

        // Decode
        let mut cursor = Cursor::new(bytes.as_slice());
        let decoded = StaticStats::decode(&mut cursor)?;

        // Validate
        assert_eq!(static_stats, decoded);

        Ok(())
    }

    #[test]
    fn timeseriesstats_encode_decode() -> anyhow::Result<()> {
        let timeseries = TimeseriesStats {
            timestamp: 123700000000000,
            equity_value: 9999999,
            percent_drawdown: 2343234,
            cumulative_return: 2343234,
            period_return: 2345432345,
        };

        let stats: Vec<TimeseriesStats> = vec![timeseries.clone(), timeseries.clone()];

        // Encode
        let mut bytes = Vec::new();
        for s in &stats {
            s.encode(&mut bytes);
        }

        // Decode
        let mut cursor = Cursor::new(bytes.as_slice());
        let mut decoded = Vec::new();

        while (cursor.position() as usize) < cursor.get_ref().len() {
            let trade = TimeseriesStats::decode(&mut cursor)?;
            decoded.push(trade);
        }

        // Validate
        assert_eq!(stats, decoded);

        Ok(())
    }

    #[test]
    fn trades_encode_decode() -> anyhow::Result<()> {
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

        let vec: Vec<Trades> = vec![trade.clone(), trade.clone()];

        // Encode all trades into a buffer
        let mut buffer = Vec::new();
        for t in &vec {
            t.encode(&mut buffer);
        }

        // Validate
        let mut cursor = Cursor::new(buffer.as_slice());
        let mut decoded = Vec::new();

        while (cursor.position() as usize) < cursor.get_ref().len() {
            let trade = Trades::decode(&mut cursor)?;
            decoded.push(trade);
        }

        // Validate
        assert_eq!(vec, decoded);

        Ok(())
    }

    #[test]
    fn signal_instructions_encode_decode() -> anyhow::Result<()> {
        let instructions = SignalInstructions {
            ticker: "AAPL".to_string(),
            order_type: "MKT".to_string(),
            action: "BUY".to_string(),
            signal_id: 1,
            weight: 13213432,
            quantity: 2343,
            limit_price: "12341".to_string(),
            aux_price: "1233212".to_string(),
        };

        let vec: Vec<SignalInstructions> = vec![instructions.clone(), instructions.clone()];

        // Encode all timeseries stats into a buffer
        let mut buffer = Vec::new();
        for s in &vec {
            s.encode(&mut buffer);
        }

        // Validate
        let mut cursor = Cursor::new(buffer.as_slice());
        let mut decoded = Vec::new();

        while (cursor.position() as usize) < cursor.get_ref().len() {
            let trade = SignalInstructions::decode(&mut cursor)?;
            decoded.push(trade);
        }

        // Validate
        assert_eq!(vec, decoded);

        Ok(())
    }

    #[test]
    fn signals_encode_decode() -> anyhow::Result<()> {
        let instructions = SignalInstructions {
            ticker: "AAPL".to_string(),
            order_type: "MKT".to_string(),
            action: "BUY".to_string(),
            signal_id: 1,
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

        // Encode all timeseries stats into a buffer
        let mut buffer = Vec::new();
        for s in &signals {
            s.encode(&mut buffer);
        }

        // Decode
        let mut cursor = Cursor::new(buffer.as_slice());
        let mut decoded = Vec::new();

        while (cursor.position() as usize) < cursor.get_ref().len() {
            let trade = Signals::decode(&mut cursor)?;
            decoded.push(trade);
        }

        // Validate
        assert_eq!(signals, decoded);

        Ok(())
    }

    #[test]
    fn backtestmetdata_encode_decode() -> anyhow::Result<()> {
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

        // Encode
        let mut buffer = Vec::new();
        bt_metadata.encode(&mut buffer);

        // Decode
        let mut cursor = Cursor::new(buffer.as_slice());
        let mut decoded = Vec::new();
        decoded.extend(BacktestMetaData::decode(&mut cursor));

        // Validate
        assert_eq!(bt_metadata, decoded[0]);

        Ok(())
    }
}
