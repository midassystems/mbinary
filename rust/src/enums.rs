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
pub enum Dataset {
    Futures = 1,
    Equities = 2,
    Option = 3,
}

impl From<Dataset> for i8 {
    fn from(dataset: Dataset) -> i8 {
        dataset as i8
    }
}

// Implement TryFrom<u8> for Dataset
impl TryFrom<i8> for Dataset {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self> {
        match value {
            1 => Ok(Dataset::Futures),
            2 => Ok(Dataset::Equities),
            3 => Ok(Dataset::Option),
            _ => Err(Error::CustomError("Invalid value for Dataset".into())),
        }
    }
}

impl Dataset {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Dataset::Futures => "futures",
            Dataset::Equities => "equities",
            Dataset::Option => "option",
        }
    }
}

impl FromStr for Dataset {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "futures" => Ok(Dataset::Futures),
            "equities" => Ok(Dataset::Equities),
            "option" => Ok(Dataset::Option),
            _ => Err(Error::CustomError(format!(
                "Unknown Dataset value: '{}'",
                value
            ))),
        }
    }
}

impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dataset::Futures => write!(f, "futures"),
            Dataset::Equities => write!(f, "equities"),
            Dataset::Option => write!(f, "option"),
        }
    }
}

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive,
)]
pub enum Stype {
    Raw = 1,
    Continuous = 2,
}

impl Stype {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Stype::Raw => "raw",
            Stype::Continuous => "continuous",
        }
    }
}

impl FromStr for Stype {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "raw" => Ok(Stype::Raw),
            "continuous" => Ok(Stype::Continuous),
            _ => Err(Error::CustomError(format!(
                "Unknown Stype value: '{}'",
                value
            ))),
        }
    }
}

impl fmt::Display for Stype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stype::Raw => write!(f, "raw"),
            Stype::Continuous => write!(f, "continuous"),
        }
    }
}

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
pub enum Side {
    Ask = b'A',
    Bid = b'B',
    None = b'N',
}

/// Handles the converting of variant to type char
impl From<Side> for char {
    fn from(side: Side) -> Self {
        u8::from(side) as char
    }
}

impl Into<i8> for Side {
    fn into(self) -> i8 {
        self as i8
    }
}

// Outputs the side as the character
// Ask == A
// Bid == B
// None == N
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Action {
    /// An existing order was modified: price and/or size.
    Modify = b'M',
    /// An aggressing order traded. Does not affect the book.
    Trade = b'T',
    /// An existing order was filled. Does not affect the book.
    Fill = b'F',
    /// An order was fully or partially cancelled.
    Cancel = b'C',
    /// A new order was added to the book.
    Add = b'A',
    /// Reset the book; clear all orders for an instrument.
    Clear = b'R',
}

// Handles the converting of variant to type char
impl From<Action> for char {
    fn from(action: Action) -> Self {
        u8::from(action) as char
    }
}

impl Into<i8> for Action {
    fn into(self) -> i8 {
        self as i8
    }
}

// Outputs the side as the character
// Modify == M
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive,
)]
pub enum Schema {
    Mbp1 = 1,
    Ohlcv1S = 2,
    Ohlcv1M = 3,
    Ohlcv1H = 4,
    Ohlcv1D = 5,
    Trades = 6,
    Tbbo = 7,
    Bbo1S = 8,
    Bbo1M = 9,
}

impl Schema {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Schema::Mbp1 => "mbp-1",
            Schema::Ohlcv1S => "ohlcv-1s",
            Schema::Ohlcv1M => "ohlcv-1m",
            Schema::Ohlcv1H => "ohlcv-1h",
            Schema::Ohlcv1D => "ohlcv-1d",
            Schema::Trades => "trades",
            Schema::Tbbo => "tbbo",
            Schema::Bbo1S => "bbo-1s",
            Schema::Bbo1M => "bbo-1m",
        }
    }
}

impl FromStr for Schema {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "mbp-1" => Ok(Schema::Mbp1),
            "ohlcv-1s" => Ok(Schema::Ohlcv1S),
            "ohlcv-1m" => Ok(Schema::Ohlcv1M),
            "ohlcv-1h" => Ok(Schema::Ohlcv1H),
            "ohlcv-1d" => Ok(Schema::Ohlcv1D),
            "trades" => Ok(Schema::Trades),
            "tbbo" => Ok(Schema::Tbbo),
            "bbo-1s" => Ok(Schema::Bbo1S),
            "bbo-1m" => Ok(Schema::Bbo1M),
            _ => Err(Error::Conversion(format!(
                "Unknown Schema value: '{}'",
                value
            ))),
        }
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schema::Mbp1 => write!(f, "mbp-1"),
            Schema::Ohlcv1S => write!(f, "ohlcv-1s"),
            Schema::Ohlcv1M => write!(f, "ohlcv-1m"),
            Schema::Ohlcv1H => write!(f, "ohlcv-1h"),
            Schema::Ohlcv1D => write!(f, "ohlcv-1d"),
            Schema::Trades => write!(f, "trades"),
            Schema::Tbbo => write!(f, "tbbo"),
            Schema::Bbo1S => write!(f, "bbo-1s"),
            Schema::Bbo1M => write!(f, "bbo-1m"),
        }
    }
}

/// Enums representing record types (RType) and schemas
#[repr(u8)]
#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbinary", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[derive(Debug, PartialEq, Eq)]
pub enum RType {
    Mbp1 = 0x01,
    Ohlcv = 0x02,
    Trades = 0x03,
    Tbbo = 0x04,
    Bbo = 0x05,
}

impl RType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            RType::Mbp1 => "mbp-1",
            RType::Ohlcv => "ohlcv",
            RType::Trades => "trades",
            RType::Tbbo => "tbbo",
            RType::Bbo => "bbo",
        }
    }
}

impl TryFrom<u8> for RType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(RType::Mbp1),
            0x02 => Ok(RType::Ohlcv),
            0x03 => Ok(RType::Trades),
            0x04 => Ok(RType::Tbbo),
            0x05 => Ok(RType::Bbo),
            _ => Err(Error::Conversion(format!(
                "Unknown RType value: '{}'",
                value
            ))),
        }
    }
}

impl From<Schema> for RType {
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::Mbp1 => RType::Mbp1,
            Schema::Ohlcv1S => RType::Ohlcv,
            Schema::Ohlcv1M => RType::Ohlcv,
            Schema::Ohlcv1H => RType::Ohlcv,
            Schema::Ohlcv1D => RType::Ohlcv,
            Schema::Trades => RType::Trades,
            Schema::Tbbo => RType::Tbbo,
            Schema::Bbo1S => RType::Bbo,
            Schema::Bbo1M => RType::Bbo,
        }
    }
}

impl FromStr for RType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mbp-1" => Ok(RType::Mbp1),
            "ohlcv" => Ok(RType::Ohlcv),
            "trades" => Ok(RType::Trades),
            "tbbo" => Ok(RType::Tbbo),
            "bbo" => Ok(RType::Bbo),
            _ => Err(Error::Conversion(format!("Invalid value for RType: {}", s))),
        }
    }
}

impl fmt::Display for RType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RType::Mbp1 => write!(f, "mbp-1"),
            RType::Ohlcv => write!(f, "ohlcv"),
            RType::Trades => write!(f, "trades"),
            RType::Tbbo => write!(f, "tbbo"),
            RType::Bbo => write!(f, "bbo"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_conv() {
        let side = Side::Ask;

        // u8
        let side_int: u8 = side.into();
        assert_eq!(side_int, side as u8);

        // From u8
        let new_side = Side::try_from(side_int).unwrap();
        assert_eq!(new_side, Side::Ask);

        // char
        let side_char = char::from(side);
        assert_eq!(side_char.to_string(), "A");
    }

    #[test]
    fn test_action_conv() {
        let action = Action::Modify;

        // u8
        let action_int: u8 = action.into();
        assert_eq!(action_int, action as u8);

        // From u8
        let new_action = Action::try_from(action_int).unwrap();
        assert_eq!(new_action, Action::Modify);

        // char
        let action_char = char::from(action);
        assert_eq!(action_char.to_string(), "M");
    }

    #[test]
    fn test_schema_conv() {
        let schema = Schema::Mbp1;

        // str
        let schema_str = schema.as_str();
        assert_eq!(schema_str, "mbp-1");

        // From str
        let _: Schema = Schema::from_str(schema_str).unwrap();
    }

    #[test]
    fn test_rtype_conv() {
        let schema = Schema::Ohlcv1S;

        // From Schema
        let rtype = RType::from(schema);
        assert_eq!(rtype.as_str(), "ohlcv");

        // From u8
        let rtype = RType::try_from(0x01).unwrap();
        assert_eq!(rtype.as_str(), RType::Mbp1.as_str());

        // str
        let rtype = RType::Ohlcv;
        let rtype_str = rtype.as_str();
        assert_eq!(rtype_str, "ohlcv");

        // From str
        let _: RType = RType::from_str("ohlcv").unwrap();
    }

    #[test]
    fn test_dataset_conv() -> anyhow::Result<()> {
        let dataset = Dataset::Futures;

        // From dataset
        let dataset_int: i8 = dataset.clone().into();
        assert_eq!(dataset_int, 1);

        // From i8
        let dataset2 = Dataset::try_from(dataset_int)?;
        assert_eq!(dataset, dataset2);

        Ok(())
    }
}
