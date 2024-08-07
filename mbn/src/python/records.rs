use crate::records::{BidAskPair, Mbp1Msg, OhlcvMsg, RecordHeader};
use pyo3::prelude::*;
use std::os::raw::c_char;

#[pymethods]
impl BidAskPair {
    #[new]
    fn py_new(
        bid_px: i64,
        ask_px: i64,
        bid_sz: u32,
        ask_sz: u32,
        bid_ct: u32,
        ask_ct: u32,
    ) -> Self {
        BidAskPair {
            bid_px,
            ask_px,
            bid_sz,
            ask_sz,
            bid_ct,
            ask_ct,
        }
    }
}

#[pymethods]
impl Mbp1Msg {
    #[new]
    fn py_new(
        hd: RecordHeader,
        price: i64,
        size: u32,
        action: c_char,
        side: c_char,
        depth: u8,
        ts_recv: u64,
        ts_in_delta: i32,
        sequence: u32,
        levels: [BidAskPair; 1],
    ) -> Self {
        Mbp1Msg {
            hd,
            price,
            size,
            action,
            side,
            depth,
            ts_recv,
            ts_in_delta,
            sequence,
            levels,
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymethods]
impl OhlcvMsg {
    #[new]
    fn py_new(hd: RecordHeader, open: i64, high: i64, low: i64, close: i64, volume: u64) -> Self {
        OhlcvMsg {
            hd,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}
