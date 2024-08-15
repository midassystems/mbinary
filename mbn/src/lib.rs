pub const METADATA_LENGTH: usize = 100;
pub mod backtest;
pub mod decode;
pub mod decode_iterator;
pub mod encode;
pub mod enums;
pub mod error;
pub mod metadata;
pub mod record_enum;
pub mod record_ref;
pub mod records;
pub mod symbols;

#[cfg(feature = "python")]
pub mod python;
