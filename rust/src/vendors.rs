use crate::error::{Error, Result};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive,
)]
pub enum Vendors {
    Internal = 0,
    Databento = 1,
    Yfinance = 2,
}
impl From<Vendors> for i8 {
    fn from(vendor: Vendors) -> i8 {
        vendor as i8
    }
}

// Implement TryFrom<u8> for Dataset
impl TryFrom<i8> for Vendors {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self> {
        match value {
            0 => Ok(Vendors::Internal),
            1 => Ok(Vendors::Databento),
            2 => Ok(Vendors::Yfinance),
            _ => Err(Error::CustomError("Invalid value for Vendor".into())),
        }
    }
}

impl Vendors {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Vendors::Internal => "internal",
            Vendors::Databento => "databento",
            Vendors::Yfinance => "yfinance",
        }
    }
}

impl FromStr for Vendors {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "internal" => Ok(Vendors::Internal),
            "databento" => Ok(Vendors::Databento),
            "yfinance" => Ok(Vendors::Yfinance),
            _ => Err(Error::CustomError(format!(
                "Unknown Vendors value: '{}'",
                value
            ))),
        }
    }
}
impl fmt::Display for Vendors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vendors::Internal => write!(f, "internal"),
            Vendors::Databento => write!(f, "databento"),
            Vendors::Yfinance => write!(f, "yfinance"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VendorData {
    Internal,
    Databento(DatabentoData),
    Yfinance(YfinanceData),
}

impl VendorData {
    pub fn encode(&self) -> u64 {
        match &self {
            VendorData::Internal => return 0,
            VendorData::Databento(data) => return data.encode(),
            VendorData::Yfinance(data) => return data.encode(),
        }
    }

    pub fn decode(raw: u64, vendor: &Vendors) -> Self {
        match vendor {
            Vendors::Internal => return VendorData::Internal,
            Vendors::Databento => return VendorData::Databento(DatabentoData::decode(raw)),
            Vendors::Yfinance => return VendorData::Yfinance(YfinanceData::decode(raw)),
        }
    }
}

// Databento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatabentoData {
    pub schema: dbn::Schema,
    pub dataset: dbn::Dataset,
    pub stype: dbn::SType,
}

impl DatabentoData {
    /// Encode the fields into a little-endian `u64`
    pub fn encode(&self) -> u64 {
        (self.dataset as u64) | ((self.stype as u64) << 16) | ((self.schema as u64) << 24)
    }
    /// Decode a little-endian `u64` back into fields
    pub fn decode(encoded: u64) -> Self {
        DatabentoData {
            dataset: dbn::Dataset::try_from((encoded & 0xFF) as u16).unwrap(), // Extract bits 0–7
            stype: dbn::SType::try_from(((encoded >> 16) & 0xFF) as u8).unwrap(),
            schema: dbn::Schema::try_from(((encoded >> 24) & 0xFF) as u16).unwrap(),
        }
    }
}

// Yfinance
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum YfinanceDataset {
    Test = 1,
    Test2 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YfinanceData {
    pub schema: YfinanceDataset,
}

impl YfinanceData {
    /// Encode the fields into a little-endian `u64`
    pub fn encode(&self) -> u64 {
        self.schema as u64
    }
    /// Decode a little-endian `u64` back into fields
    pub fn decode(encoded: u64) -> Self {
        YfinanceData {
            schema: YfinanceDataset::try_from((encoded & 0xFF) as u8).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_vendors_conv() -> anyhow::Result<()> {
        let vendor = Vendors::Databento;

        // From dataset
        let vendor_int: i8 = vendor.clone().into();
        assert_eq!(vendor_int, 1);

        // From i8
        let vendor2 = Vendors::try_from(vendor_int)?;
        assert_eq!(vendor, vendor2);

        Ok(())
    }

    #[test]
    fn test_encoding_databento_data() -> anyhow::Result<()> {
        let schema = dbn::Schema::from_str("mbp-1")?;
        let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
        let stype = dbn::SType::from_str("raw_symbol")?;

        let db_data = DatabentoData {
            schema,
            dataset,
            stype,
        };
        let vendor_data = VendorData::Databento(db_data);

        // Test
        let vendor_data_int = vendor_data.encode();

        let decoded = VendorData::decode(vendor_data_int, &Vendors::Databento);

        // Validate
        assert_eq!(decoded, vendor_data);
        Ok(())
    }

    #[test]
    fn test_encoding_yfinance_data() -> anyhow::Result<()> {
        let schema = YfinanceDataset::Test;
        let data = YfinanceData { schema };

        let vendor_data = VendorData::Yfinance(data);

        // Test
        let vendor_data_int = vendor_data.encode();

        let decoded = VendorData::decode(vendor_data_int, &Vendors::Yfinance);

        // Validate
        assert_eq!(decoded, vendor_data);
        Ok(())
    }
}
