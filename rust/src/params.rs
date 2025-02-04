use crate::enums::{Dataset, RType, Schema, Stype};
use crate::utils::date_to_unix_nanos;
use crate::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RetrieveParams {
    pub symbols: Vec<String>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub schema: Schema,
    pub dataset: Dataset,
    pub stype: Stype,
}

impl RetrieveParams {
    pub fn new(
        symbols: Vec<String>,
        start: &str,
        end: &str,
        schema: Schema,
        dataset: Dataset,
        stype: Stype,
    ) -> Result<Self> {
        Ok(RetrieveParams {
            symbols,
            start_ts: date_to_unix_nanos(start)?,
            end_ts: date_to_unix_nanos(end)?,
            schema,
            dataset,
            stype,
        })
    }

    pub fn rtype(&self) -> Result<RType> {
        Ok(RType::from(self.schema))
    }

    pub fn schema_interval(&self) -> Result<i64> {
        match &self.schema {
            Schema::Mbp1 => Ok(1), // 1 nanosecond
            Schema::Trades => Ok(1),
            Schema::Tbbo => Ok(1),
            Schema::Ohlcv1S => Ok(1_000_000_000), // 1 second in nanoseconds
            Schema::Ohlcv1M => Ok(60_000_000_000), // 1 minute in nanoseconds
            Schema::Ohlcv1H => Ok(3_600_000_000_000), // 1 hour in nanoseconds
            Schema::Ohlcv1D => Ok(86_400_000_000_000), // 1 day in nanoseconds
            Schema::Bbo1S => Ok(1_000_000_000),
            Schema::Bbo1M => Ok(60_000_000_000),
        }
    }

    pub fn interval_adjust_ts_start(&mut self) -> Result<()> {
        let interval_ns = self.schema_interval()?;

        if self.start_ts % interval_ns == 0 {
            // If the timestamp is already aligned to the interval, return it as is
            Ok(())
        } else {
            self.start_ts = self.start_ts - (self.start_ts % interval_ns);
            Ok(())
        }
    }

    pub fn interval_adjust_ts_end(&mut self) -> Result<()> {
        let interval_ns = self.schema_interval()?;

        if self.end_ts % interval_ns == 0 {
            // If the timestamp is already aligned to the interval, return it as is
            Ok(())
        } else {
            // If not aligned, round up to the next interval boundary
            self.end_ts = self.end_ts + (interval_ns - (self.end_ts % interval_ns));
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_retrieve_params_schema() -> anyhow::Result<()> {
        let params = RetrieveParams {
            symbols: vec!["AAPL".to_string()],
            start_ts: 1704209103644092563,
            end_ts: 1704209903644092567,
            schema: Schema::Mbp1, //tring::from("mbp-1"),
            dataset: Dataset::Futures,
            stype: Stype::Raw,
        };

        // Test
        assert_eq!(params.schema, Schema::Mbp1);

        Ok(())
    }

    #[test]
    fn test_retrieve_params_schema_interval() -> anyhow::Result<()> {
        let params = RetrieveParams {
            symbols: vec!["AAPL".to_string()],
            start_ts: 1704209103644092563,
            end_ts: 1704209903644092567,
            schema: Schema::Tbbo, //tring::from("tbbo"),
            dataset: Dataset::Equities,
            stype: Stype::Raw,
        };

        // Test
        assert_eq!(params.schema_interval()?, 1);

        Ok(())
    }

    #[test]
    fn test_retrieve_params_rtype() -> anyhow::Result<()> {
        let params = RetrieveParams {
            symbols: vec!["AAPL".to_string()],
            start_ts: 1728878401000000000,
            end_ts: 1728878460000000000,
            schema: Schema::Ohlcv1H, //tring::from("ohlcv-1h"),
            dataset: Dataset::Option,
            stype: Stype::Raw,
        };

        // Test
        assert_eq!(params.rtype()?, RType::Ohlcv);

        Ok(())
    }
}
