pub const METADATA_LENGTH: usize = 100;
pub const PRICE_SCALE: i64 = 1_000_000_000;
pub mod backtest;
pub mod decode;
pub mod decode_iterator;
pub mod encode;
pub mod enums;
pub mod error;
pub mod live;
pub mod metadata;
pub mod record_enum;
pub mod record_ref;
pub mod records;
pub mod symbols;
pub mod utils;

#[cfg(feature = "python")]
pub mod python;
