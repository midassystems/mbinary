use crate::enums::RType;
use databento::dbn;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::From;
use std::{mem, os::raw::c_char, ptr::NonNull, slice};

#[cfg(feature = "python")]
use pyo3::pyclass;

/// Trait to access common header across records.
pub trait Record {
    fn header(&self) -> &RecordHeader;
}

/// Trait to check if a type has a specific RType property.
pub trait HasRType {
    fn has_rtype(rtype: u8) -> bool;
    fn rtype_byte() -> u8;
}

/// Constant data across all records.
#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordHeader {
    pub length: u8,
    pub rtype: u8,
    pub instrument_id: u32,
    pub ts_event: u64,
}

// Implementing Send and Sync for RecordHeader
unsafe impl Send for RecordHeader {}
unsafe impl Sync for RecordHeader {}

impl RecordHeader {
    // Allows length to remaind u8 regardless of size
    pub const LENGTH_MULTIPLIER: usize = 4;

    pub fn new<R: HasRType>(instrument_id: u32, ts_event: u64) -> Self {
        Self {
            length: (mem::size_of::<R>() / Self::LENGTH_MULTIPLIER) as u8,
            rtype: R::rtype_byte(),
            instrument_id,
            ts_event,
        }
    }

    pub const fn record_size(&self) -> usize {
        self.length as usize * Self::LENGTH_MULTIPLIER
    }

    pub fn rtype(&self) -> RType {
        RType::try_from(self.rtype).unwrap()
    }

    pub fn from_dbn<R: HasRType>(header: dbn::RecordHeader) -> Self {
        RecordHeader::new::<R>(header.instrument_id, header.ts_event)
    }
}

/// Order book level e.g. MBP1 would contain the top level.
#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct BidAskPair {
    /// The bid price.
    pub bid_px: i64,
    /// The ask price.
    pub ask_px: i64,
    /// The bid size.
    pub bid_sz: u32,
    /// The ask size.
    pub ask_sz: u32,
    /// The bid order count.
    pub bid_ct: u32,
    /// The ask order count.
    pub ask_ct: u32,
}

impl From<dbn::BidAskPair> for BidAskPair {
    fn from(dbn_pair: dbn::BidAskPair) -> Self {
        BidAskPair {
            bid_px: dbn_pair.bid_px,
            ask_px: dbn_pair.ask_px,
            bid_sz: dbn_pair.bid_sz,
            ask_sz: dbn_pair.ask_sz,
            bid_ct: dbn_pair.bid_ct,
            ask_ct: dbn_pair.ask_ct,
        }
    }
}

/// Mbp1Msg struct
#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct Mbp1Msg {
    pub hd: RecordHeader,
    pub price: i64,
    pub size: u32,
    pub action: c_char,
    pub side: c_char,
    pub depth: u8,
    pub flags: u8,
    pub ts_recv: u64,
    pub ts_in_delta: i32,
    pub sequence: u32,
    pub levels: [BidAskPair; 1],
}

impl Record for Mbp1Msg {
    fn header(&self) -> &RecordHeader {
        &self.hd
    }
}

impl HasRType for Mbp1Msg {
    fn has_rtype(rtype: u8) -> bool {
        rtype == RType::Mbp1 as u8
    }

    fn rtype_byte() -> u8 {
        RType::Mbp1 as u8
    }
}

impl AsRef<[u8]> for Mbp1Msg {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Mbp1Msg) as *const u8,
                mem::size_of::<Mbp1Msg>(),
            )
        }
    }
}
impl From<dbn::Mbp1Msg> for Mbp1Msg {
    fn from(item: dbn::Mbp1Msg) -> Self {
        Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(item.hd.instrument_id, item.hd.ts_event),
            price: item.price,
            size: item.size,
            action: item.action,
            side: item.side,
            depth: item.depth,
            flags: item.flags.raw(),
            ts_recv: item.ts_recv,
            ts_in_delta: item.ts_in_delta,
            sequence: item.sequence,
            levels: [BidAskPair::from(item.levels[0].clone())],
        }
    }
}

/// OhlcvMsg struct
#[repr(C)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OhlcvMsg {
    pub hd: RecordHeader,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub volume: u64,
}

impl Record for OhlcvMsg {
    fn header(&self) -> &RecordHeader {
        &self.hd
    }
}

impl HasRType for OhlcvMsg {
    fn has_rtype(rtype: u8) -> bool {
        rtype == RType::Ohlcv as u8
    }

    fn rtype_byte() -> u8 {
        RType::Ohlcv as u8
    }
}

impl AsRef<[u8]> for OhlcvMsg {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const OhlcvMsg) as *const u8,
                mem::size_of::<OhlcvMsg>(),
            )
        }
    }
}

/// Transmutes entire byte slices header and record
pub unsafe fn transmute_record_bytes<T: HasRType>(bytes: &[u8]) -> Option<T> {
    assert!(
        bytes.len() >= mem::size_of::<T>(),
        "Passing a slice smaller than `{}` to `transmute_record_bytes_owned` is invalid",
        std::any::type_name::<T>()
    );
    let non_null = NonNull::new_unchecked(bytes.as_ptr().cast_mut());
    if T::has_rtype(non_null.cast::<RecordHeader>().as_ref().rtype) {
        Some(non_null.cast::<T>().as_ptr().read())
    } else {
        None
    }
}

// Transmutes header from byte slice
pub unsafe fn transmute_header_bytes(bytes: &[u8]) -> Option<&RecordHeader> {
    assert!(
        bytes.len() >= mem::size_of::<RecordHeader>(),
        concat!(
            "Passing a slice smaller than `",
            stringify!(RecordHeader),
            "` to `transmute_header_bytes` is invalid"
        )
    );
    let non_null = NonNull::new_unchecked(bytes.as_ptr().cast_mut());
    let header = non_null.cast::<RecordHeader>().as_ref();
    if header.record_size() > bytes.len() {
        None
    } else {
        Some(header)
    }
}

// Transmutes record from an already transmuted header
pub unsafe fn transmute_record<T: HasRType>(header: &RecordHeader) -> Option<&T> {
    if T::has_rtype(header.rtype) {
        // Safety: because it comes from a reference, `header` must not be null. It's ok
        // to cast to `mut` because it's never mutated.
        let non_null = NonNull::from(header);
        Some(non_null.cast::<T>().as_ref())
    } else {
        None
    }
}

// Creates byte slice of a record
#[allow(dead_code)]
pub(crate) unsafe fn as_u8_slice<T: Sized>(data: &T) -> &[u8] {
    slice::from_raw_parts((data as *const T).cast(), mem::size_of::<T>())
}

#[cfg(test)]
mod tests {
    use crate::enums::{Action, Side};

    use super::*;

    #[test]
    fn test_construct_record() {
        // Test
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
            price: 1000,
            size: 10,
            action: Action::Modify.into(),
            side: Side::Bid.into(),
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            levels: [BidAskPair {
                bid_px: 1,
                ask_px: 2,
                bid_sz: 2,
                ask_sz: 2,
                bid_ct: 1,
                ask_ct: 3,
            }],
        };

        // Validate
        let rtype_u8 = record.header().rtype;
        let rtype = RType::try_from(rtype_u8).unwrap();
        assert_eq!(rtype.as_str(), "mbp-1");
    }

    #[test]
    fn test_record_header_transmute() {
        // Test
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
            price: 1000,
            size: 10,
            action: 1,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            levels: [BidAskPair {
                bid_px: 1,
                ask_px: 2,
                bid_sz: 2,
                ask_sz: 2,
                bid_ct: 1,
                ask_ct: 3,
            }],
        };

        let bytes = record.as_ref();

        // Validate
        let decoded_header: &RecordHeader = unsafe { transmute_header_bytes(bytes).unwrap() };
        assert_eq!(decoded_header.record_size(), std::mem::size_of::<Mbp1Msg>());
    }

    #[test]
    fn test_transmute_record() {
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
            price: 1000,
            size: 10,
            action: Action::Add.into(),
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            levels: [BidAskPair {
                bid_px: 1,
                ask_px: 2,
                bid_sz: 2,
                ask_sz: 2,
                bid_ct: 1,
                ask_ct: 3,
            }],
        };

        // Test
        let bytes = record.as_ref();
        let decoded_header: &RecordHeader = unsafe { transmute_header_bytes(bytes).unwrap() };

        // Validate
        let out: &Mbp1Msg = unsafe { transmute_record(decoded_header).unwrap() };
        assert_eq!(out, &record);
    }

    #[test]
    fn test_transmute_record_bytes() {
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1725734014000000000),
            price: 1000,
            size: 10,
            action: Action::Trade as i8,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 1725734014000000000,
            ts_in_delta: 12345,
            sequence: 123456,
            levels: [BidAskPair {
                bid_px: 1,
                ask_px: 2,
                bid_sz: 2,
                ask_sz: 2,
                bid_ct: 1,
                ask_ct: 3,
            }],
        };

        // Test
        let bytes = unsafe { as_u8_slice(&record) };

        // Test
        let decoded_record: Mbp1Msg = unsafe { transmute_record_bytes(bytes).unwrap() };
        assert_eq!(decoded_record, record);
    }
}
