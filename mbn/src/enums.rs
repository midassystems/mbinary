use crate::error::{Error, Result};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;
use std::str::FromStr;
use std::{ffi::CStr, mem, os::raw::c_char, ptr::NonNull, slice};

#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbn", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
pub enum Side {
    /// A sell order or sell aggressor in a trade.
    Ask = b'A',
    /// A buy order or a buy aggressor in a trade.
    Bid = b'B',
    /// No side specified by the original source.
    None = b'N',
}

// Handles the converting of variant to type char
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
    pyclass(module = "mbn", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
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

/// Constants for the bit flag record fields.
pub mod flags {
    /// Indicates it's the last message in the packet from the venue for a given
    /// `instrument_id`.
    pub const LAST: u8 = 1 << 7;
    /// Indicates a top-of-book message, not an individual order.
    pub const TOB: u8 = 1 << 6;
    /// Indicates the message was sourced from a replay, such as a snapshot server.
    pub const SNAPSHOT: u8 = 1 << 5;
    /// Indicates an aggregated price level message, not an individual order.
    pub const MBP: u8 = 1 << 4;
    /// Indicates the `ts_recv` value is inaccurate due to clock issues or packet
    /// reordering.
    pub const BAD_TS_RECV: u8 = 1 << 3;
    /// Indicates an unrecoverable gap was detected in the channel.
    pub const MAYBE_BAD_BOOK: u8 = 1 << 2;
}

#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbn", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum Schema {
    Mbp1 = 1,
    Ohlcv1S = 2,
    Ohlcv1M = 3,
    Ohlcv1H = 4,
    Ohlcv1D = 5,
}

impl Schema {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Schema::Mbp1 => "mbp-1",
            Schema::Ohlcv1S => "ohlcv-1s",
            Schema::Ohlcv1M => "ohlcv-1m",
            Schema::Ohlcv1H => "ohlcv-1h",
            Schema::Ohlcv1D => "ohlcv-1d",
        }
    }

    // pub fn base_name(&self) -> &'static str {
    //     match self {
    //         Schema::Mbp1 => "mbp",
    //         Schema::Ohlcv1S => "ohlcv",
    //         Schema::Ohlcv1M => "ohlcv",
    //     }
    // }
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
        }
    }
}

/// Enums representing record types (RType) and schemas
#[cfg_attr(feature = "python", derive(strum::EnumIter, strum::AsRefStr))]
#[cfg_attr(
    feature = "python",
    pyclass(module = "mbn", rename_all = "SCREAMING_SNAKE_CASE", eq, eq_int)
)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum RType {
    Mbp1 = 0x01,
    Ohlcv = 0x02,
}

impl RType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            RType::Mbp1 => "mbp-1",
            RType::Ohlcv => "ohlcv",
        }
    }
}

impl TryFrom<u8> for RType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(RType::Mbp1),
            0x02 => Ok(RType::Ohlcv),
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
        }
    }
}

impl FromStr for RType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mbp-1" => Ok(RType::Mbp1),
            "ohlcv" => Ok(RType::Ohlcv),
            _ => Err(Error::Conversion(format!("Invalid value for RType: {}", s))),
        }
    }
}

impl fmt::Display for RType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RType::Mbp1 => write!(f, "mbp-1"),
            RType::Ohlcv => write!(f, "ohlcv"),
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
}
