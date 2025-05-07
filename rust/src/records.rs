use crate::enums::RType;
use crate::PRICE_SCALE;
use dbn;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::From;
use std::{mem, os::raw::c_char, ptr::NonNull, slice};

#[cfg(feature = "python")]
use pyo3::pyclass;

/// Trait to access common header across records.
pub trait Record {
    fn header(&self) -> &RecordHeader;
    fn timestamp(&self) -> u64;
    fn price(&self) -> i64;
}

/// Trait to check if a type has a specific RType property.
pub trait HasRType {
    fn has_rtype(rtype: u8) -> bool;
    fn rtype_byte() -> u8;
}

/// Constant data across all records.
#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordHeader {
    pub length: u8,
    pub rtype: u8,
    pub instrument_id: u32,
    pub ts_event: u64,
    pub rollover_flag: u8,
}

// Implementing Send and Sync for RecordHeader
unsafe impl Send for RecordHeader {}
unsafe impl Sync for RecordHeader {}

impl RecordHeader {
    // Allows length to remaind u8 regardless of size
    pub const LENGTH_MULTIPLIER: usize = 4;

    pub fn new<R: HasRType>(instrument_id: u32, ts_event: u64, rollover_flag: u8) -> Self {
        Self {
            length: (mem::size_of::<R>() / Self::LENGTH_MULTIPLIER) as u8,
            rtype: R::rtype_byte(),
            instrument_id,
            ts_event,
            rollover_flag,
        }
    }

    pub const fn record_size(&self) -> usize {
        self.length as usize * Self::LENGTH_MULTIPLIER
    }

    pub fn rtype(&self) -> RType {
        RType::try_from(self.rtype).unwrap()
    }

    pub fn from_dbn<R: HasRType>(header: dbn::RecordHeader) -> Self {
        RecordHeader::new::<R>(header.instrument_id, header.ts_event, 0)
    }
}

/// Order book level e.g. MBP1 would contain the top level.
#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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

impl BidAskPair {
    pub fn mid_px(&self) -> i64 {
        let bid_px: f64 = self.bid_px as f64 / PRICE_SCALE as f64;
        let ask_px: f64 = self.ask_px as f64 / PRICE_SCALE as f64;
        let mid: f64 = (bid_px + ask_px) / 2.0 * PRICE_SCALE as f64;
        mid as i64
    }
    pub fn pretty_mid_px(&self) -> f64 {
        let bid_px: f64 = (self.bid_px / PRICE_SCALE) as f64;
        let ask_px: f64 = (self.ask_px / PRICE_SCALE) as f64;
        println!("{:?} {:?}", bid_px, ask_px);
        let mid = (bid_px + ask_px) / 2.0;
        println!("{:?}", mid);
        mid
    }
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

impl PartialEq<dbn::BidAskPair> for BidAskPair {
    fn eq(&self, other: &dbn::BidAskPair) -> bool {
        self.bid_px == other.bid_px
            && self.ask_px == other.ask_px
            && self.bid_sz == other.bid_sz
            && self.ask_sz == other.ask_sz
            && self.bid_ct == other.bid_ct
            && self.ask_ct == other.ask_ct
    }
}

/// Mbp1Msg struct
#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
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
    /// Differentiates records that are otherwise the same but not duplicates.
    pub discriminator: u32,
    pub levels: [BidAskPair; 1],
}

impl Record for Mbp1Msg {
    fn header(&self) -> &RecordHeader {
        &self.hd
    }
    fn timestamp(&self) -> u64 {
        self.ts_recv
    }
    fn price(&self) -> i64 {
        self.price
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
            hd: RecordHeader::new::<Mbp1Msg>(item.hd.instrument_id, item.hd.ts_event, 0),
            price: item.price,
            size: item.size,
            action: item.action,
            side: item.side,
            depth: item.depth,
            flags: item.flags.raw(),
            ts_recv: item.ts_recv,
            ts_in_delta: item.ts_in_delta,
            sequence: item.sequence,
            discriminator: 0,
            levels: [BidAskPair::from(item.levels[0].clone())],
        }
    }
}

impl From<&dbn::Mbp1Msg> for Mbp1Msg {
    fn from(item: &dbn::Mbp1Msg) -> Self {
        Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(item.hd.instrument_id, item.hd.ts_event, 0),
            price: item.price,
            size: item.size,
            action: item.action,
            side: item.side,
            depth: item.depth,
            flags: item.flags.raw(),
            ts_recv: item.ts_recv,
            ts_in_delta: item.ts_in_delta,
            sequence: item.sequence,
            discriminator: 0,
            levels: [BidAskPair::from(item.levels[0].clone())],
        }
    }
}

impl PartialEq<dbn::Mbp1Msg> for Mbp1Msg {
    fn eq(&self, other: &dbn::Mbp1Msg) -> bool {
        self.hd.ts_event == other.hd.ts_event
            && self.price == other.price
            && self.size == other.size
            && self.action == other.action
            && self.side == other.side
            && self.depth == other.depth
            && self.ts_recv == other.ts_recv
            && self.ts_in_delta == other.ts_in_delta
            && self.sequence == other.sequence
            && self.levels[0] == other.levels[0]
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct TradeMsg {
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
}

impl Record for TradeMsg {
    fn header(&self) -> &RecordHeader {
        &self.hd
    }
    fn timestamp(&self) -> u64 {
        self.ts_recv
    }
    fn price(&self) -> i64 {
        self.price
    }
}

impl HasRType for TradeMsg {
    fn has_rtype(rtype: u8) -> bool {
        rtype == RType::Trades as u8
    }

    fn rtype_byte() -> u8 {
        RType::Trades as u8
    }
}

impl AsRef<[u8]> for TradeMsg {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const TradeMsg) as *const u8,
                mem::size_of::<TradeMsg>(),
            )
        }
    }
}
impl From<dbn::TradeMsg> for TradeMsg {
    fn from(item: dbn::TradeMsg) -> Self {
        TradeMsg {
            hd: RecordHeader::new::<TradeMsg>(item.hd.instrument_id, item.hd.ts_event, 0),
            price: item.price,
            size: item.size,
            action: item.action,
            side: item.side,
            depth: item.depth,
            flags: item.flags.raw(),
            ts_recv: item.ts_recv,
            ts_in_delta: item.ts_in_delta,
            sequence: item.sequence,
        }
    }
}

impl PartialEq<dbn::TradeMsg> for TradeMsg {
    fn eq(&self, other: &dbn::TradeMsg) -> bool {
        self.hd.ts_event == other.hd.ts_event
            && self.price == other.price
            && self.size == other.size
            && self.action == other.action
            && self.side == other.side
            && self.depth == other.depth
            && self.ts_recv == other.ts_recv
            && self.ts_in_delta == other.ts_in_delta
            && self.sequence == other.sequence
    }
}

#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct BboMsg {
    pub hd: RecordHeader,
    pub levels: [BidAskPair; 1],
}

impl Record for BboMsg {
    fn header(&self) -> &RecordHeader {
        &self.hd
    }
    fn timestamp(&self) -> u64 {
        self.hd.ts_event
    }
    fn price(&self) -> i64 {
        self.levels[0].mid_px()
    }
}

impl HasRType for BboMsg {
    fn has_rtype(rtype: u8) -> bool {
        rtype == RType::Bbo as u8
    }

    fn rtype_byte() -> u8 {
        RType::Bbo as u8
    }
}

impl AsRef<[u8]> for BboMsg {
    fn as_ref(&self) -> &[u8] {
        unsafe { as_u8_slice(self) }
    }
}

impl From<dbn::BboMsg> for BboMsg {
    fn from(item: dbn::BboMsg) -> Self {
        BboMsg {
            hd: RecordHeader::new::<BboMsg>(item.hd.instrument_id, item.hd.ts_event, 0),
            levels: [BidAskPair::from(item.levels[0].clone())],
        }
    }
}

impl From<dbn::Mbp1Msg> for BboMsg {
    fn from(item: dbn::Mbp1Msg) -> Self {
        BboMsg {
            hd: RecordHeader::new::<BboMsg>(item.hd.instrument_id, item.hd.ts_event, 0),
            levels: [BidAskPair::from(item.levels[0].clone())],
        }
    }
}

impl PartialEq<dbn::Mbp1Msg> for BboMsg {
    fn eq(&self, other: &dbn::Mbp1Msg) -> bool {
        self.hd.ts_event == other.ts_recv && self.levels[0] == other.levels[0]
    }
}

/// TBBO is jsut MBP1 where action is always Trade
pub type TbboMsg = Mbp1Msg;

/// OhlcvMsg struct
#[repr(C)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, dict, module = "mbinary")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    fn timestamp(&self) -> u64 {
        self.header().ts_event
    }
    fn price(&self) -> i64 {
        self.close
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
impl From<dbn::OhlcvMsg> for OhlcvMsg {
    fn from(item: dbn::OhlcvMsg) -> Self {
        OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(item.hd.instrument_id, item.hd.ts_event, 0),
            open: item.open,
            high: item.high,
            low: item.low,
            close: item.close,
            volume: item.volume,
        }
    }
}

impl PartialEq<dbn::OhlcvMsg> for OhlcvMsg {
    fn eq(&self, other: &dbn::OhlcvMsg) -> bool {
        self.hd.ts_event == other.hd.ts_event
            && self.open == other.open
            && self.high == other.high
            && self.low == other.low
            && self.close == other.close
            && self.volume == other.volume
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

    use super::*;
    use crate::enums::{Action, Side};
    use dbn::FlagSet;

    #[test]
    fn test_construct_record() {
        // Test
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124, 0),
            price: 1000,
            size: 10,
            action: Action::Modify.into(),
            side: Side::Bid.into(),
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            discriminator: 0,
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
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124, 0),
            price: 1000,
            size: 10,
            action: 1,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            discriminator: 0,
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
            hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124, 0),
            price: 1000,
            size: 10,
            action: Action::Add.into(),
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 123456789098765,
            ts_in_delta: 12345,
            sequence: 123456,
            discriminator: 0,
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
    fn test_transmute_record_mbp() {
        let record = Mbp1Msg {
            hd: RecordHeader::new::<Mbp1Msg>(1, 1725734014000000000, 0),
            price: 1000,
            size: 10,
            action: Action::Trade as i8,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 1725734014000000000,
            ts_in_delta: 12345,
            sequence: 123456,
            discriminator: 0,
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

        // Validate
        let decoded_record: Mbp1Msg = unsafe { transmute_record_bytes(bytes).unwrap() };
        assert_eq!(decoded_record, record);
    }

    #[test]
    fn test_transmute_record_trade() {
        let record = TradeMsg {
            hd: RecordHeader::new::<TradeMsg>(1, 1725734014000000000, 0),
            price: 1000,
            size: 10,
            action: Action::Trade as i8,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 1725734014000000000,
            ts_in_delta: 12345,
            sequence: 123456,
        };

        // Test
        let bytes = record.as_ref();

        // Validate
        let decoded_record: TradeMsg = unsafe { transmute_record_bytes(bytes).unwrap() };
        assert_eq!(decoded_record, record);
    }

    #[test]
    fn test_transmute_record_tbbo() {
        let record = TbboMsg {
            hd: RecordHeader::new::<TbboMsg>(1, 1725734014000000000, 0),
            price: 1000,
            size: 10,
            action: Action::Trade as i8,
            side: 1,
            depth: 0,
            flags: 0,
            ts_recv: 1725734014000000000,
            ts_in_delta: 12345,
            sequence: 123456,
            discriminator: 0,
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

        // Validate
        let decoded_record: TbboMsg = unsafe { transmute_record_bytes(bytes).unwrap() };
        assert_eq!(decoded_record, record);
    }

    #[test]
    fn test_transmute_record_bbo() {
        let record = BboMsg {
            hd: RecordHeader::new::<BboMsg>(1, 1725734014000000000, 0),
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

        // Validate
        let decoded_record: BboMsg = unsafe { transmute_record_bytes(bytes).unwrap() };
        assert_eq!(decoded_record, record);
    }

    #[test]
    fn bidaskpair_eq() -> anyhow::Result<()> {
        let dbn_pair = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let mbinary_pair = BidAskPair::from(dbn_pair.clone());

        // Test
        assert!(mbinary_pair == dbn_pair);

        Ok(())
    }

    #[test]
    fn bidaskpair_ineq() -> anyhow::Result<()> {
        let dbn_pair = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let mut mbinary_pair = BidAskPair::from(dbn_pair.clone());
        mbinary_pair.bid_px = 12343212;

        // Test
        assert!(mbinary_pair != dbn_pair);

        Ok(())
    }

    #[test]
    fn mbp1_eq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::Mbp1Msg>(1, 1231, 1231, 1700000000000000);
        let bid_ask = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let dbn_mbp = dbn::Mbp1Msg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
            levels: [bid_ask],
        };

        // Test
        let mbinary_mbp = Mbp1Msg::from(dbn_mbp.clone());
        assert!(mbinary_mbp == dbn_mbp);

        Ok(())
    }

    #[test]
    fn mbp1_ineq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::Mbp1Msg>(1, 1231, 1231, 1700000000000000);
        let bid_ask = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let dbn_mbp = dbn::Mbp1Msg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
            levels: [bid_ask],
        };

        // Test
        let mut mbinary_mbp = Mbp1Msg::from(dbn_mbp.clone());
        mbinary_mbp.price = 123432343234323;
        assert!(mbinary_mbp != dbn_mbp);

        Ok(())
    }

    #[test]
    fn trades_eq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::TradeMsg>(1, 1231, 1231, 1700000000000000);

        let dbn_record = dbn::TradeMsg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
        };

        // Test
        let mbinary_record = TradeMsg::from(dbn_record.clone());
        assert!(mbinary_record == dbn_record);

        Ok(())
    }

    #[test]
    fn trades_ineq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::TradeMsg>(1, 1231, 1231, 1700000000000000);

        let dbn_record = dbn::TradeMsg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
        };

        // Test
        let mut mbinary_record = TradeMsg::from(dbn_record.clone());
        mbinary_record.price = 123432343234323;
        assert!(mbinary_record != dbn_record);

        Ok(())
    }
    #[test]
    fn bbo_eq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::BboMsg>(1, 1231, 1231, 1700000000000000);

        let bid_ask = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let dbn_record = dbn::Mbp1Msg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
            levels: [bid_ask],
        };

        // Test
        let mut mbinary_record = BboMsg::from(dbn_record.clone());
        mbinary_record.hd.ts_event = 1231;

        assert!(mbinary_record == dbn_record);

        Ok(())
    }

    #[test]
    fn bbo_eq_undef_price() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::BboMsg>(1, 1231, 1231, 1700000000000000);

        let bid_ask = dbn::BidAskPair {
            bid_px: dbn::UNDEF_PRICE,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let dbn_record = dbn::Mbp1Msg {
            hd: header,
            price: dbn::UNDEF_PRICE,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
            levels: [bid_ask],
        };

        // Test
        let mut mbinary_record = BboMsg::from(dbn_record.clone());
        mbinary_record.hd.ts_event = 1231;

        assert!(mbinary_record == dbn_record);

        Ok(())
    }

    #[test]
    fn bbo_ineq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::BboMsg>(1, 1231, 1231, 1700000000000000);

        let bid_ask = dbn::BidAskPair {
            bid_px: 10000000,
            ask_px: 200000,
            bid_sz: 3000000,
            ask_sz: 400000000,
            bid_ct: 50000000,
            ask_ct: 60000000,
        };

        let dbn_record = dbn::Mbp1Msg {
            hd: header,
            price: 12345676543,
            size: 1234543,
            action: 0,
            side: 0,
            flags: FlagSet::empty(),
            depth: 10,
            ts_recv: 1231,
            ts_in_delta: 123432,
            sequence: 23432,
            levels: [bid_ask],
        };

        // Test
        let mut mbinary_record = BboMsg::from(dbn_record.clone());
        mbinary_record.levels[0].bid_px = 123432343234323;
        assert!(mbinary_record != dbn_record);

        Ok(())
    }
    #[test]
    fn ohlcv_eq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::OhlcvMsg>(1, 1231, 1231, 1700000000000000);

        let dbn_record = dbn::OhlcvMsg {
            hd: header,
            open: 1232123,
            high: 234323432,
            low: 1234212343,
            close: 1234312345,
            volume: 12342134,
        };

        // Test
        let mbinary_record = OhlcvMsg::from(dbn_record.clone());
        assert!(mbinary_record == dbn_record);

        Ok(())
    }

    #[test]
    fn ohlcv_ineq() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::OhlcvMsg>(1, 1231, 1231, 1700000000000000);

        let dbn_record = dbn::OhlcvMsg {
            hd: header,
            open: 234323423,
            high: 23443234,
            low: 231342134,
            close: 32432321343,
            volume: 12342134321,
        };

        // Test
        let mut mbinary_record = OhlcvMsg::from(dbn_record.clone());
        mbinary_record.open = 123432343234323;
        assert!(mbinary_record != dbn_record);

        Ok(())
    }

    #[test]
    fn bbo_eq_undef_bidaspair_px() -> anyhow::Result<()> {
        let header = dbn::RecordHeader::new::<dbn::BboMsg>(1, 1, 42009846, 1704183584805953819);

        let bid_ask = dbn::BidAskPair {
            bid_px: dbn::UNDEF_PRICE,
            ask_px: 2220000000000,
            bid_sz: 0,
            ask_sz: 2,
            bid_ct: 0,
            ask_ct: 1,
        };

        let dbn_record = dbn::Mbp1Msg {
            hd: header,
            price: 2073700000000,
            size: 3,
            action: 0,
            side: 66,
            flags: FlagSet::empty(),
            depth: 0,
            ts_recv: 1704183600000000000,
            ts_in_delta: 0,
            sequence: 294640,
            levels: [bid_ask],
        };

        // Test
        let mut mbinary_record = BboMsg::from(dbn_record.clone());
        mbinary_record.hd.ts_event = 1704183600000000000;

        // Validate
        assert!(mbinary_record == dbn_record);

        Ok(())
    }
}
