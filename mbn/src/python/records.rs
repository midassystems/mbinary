use crate::record_enum::RecordEnum;
use crate::records::{BidAskPair, Mbp1Msg, OhlcvMsg, Record, RecordHeader};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::os::raw::c_char;

#[cfg_attr(feature = "python", pyclass(dict, module = "mbn"))]
pub struct RecordMsg {
    inner: RecordEnum,
}

#[pymethods]
impl RecordMsg {
    #[getter]
    fn hd(&self) -> RecordHeader {
        self.inner.header().clone()
    }

    #[getter]
    fn price(&self) -> i64 {
        self.inner.price()
    }
}

#[pymethods]
impl RecordHeader {
    #[getter]
    fn ts_event(&self) -> u64 {
        self.ts_event
    }

    #[getter]
    fn instrument_id(&self) -> u32 {
        self.instrument_id
    }
}

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

    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("price", self.price).unwrap();
        dict.set_item("size", self.size).unwrap();
        dict.set_item("action", self.action).unwrap();
        dict.set_item("side", self.side).unwrap();
        dict.set_item("depth", self.depth).unwrap();
        dict.set_item("ts_recv", self.ts_recv).unwrap();
        dict.set_item("ts_in_delta", self.ts_in_delta).unwrap();
        dict.set_item("sequence", self.sequence).unwrap();
        dict.set_item("bid_px", self.levels[0].bid_px).unwrap();
        dict.set_item("ask_px", self.levels[0].ask_px).unwrap();
        dict.set_item("bid_sz", self.levels[0].bid_sz).unwrap();
        dict.set_item("ask_sz", self.levels[0].ask_sz).unwrap();
        dict.set_item("bid_ct", self.levels[0].bid_ct).unwrap();
        dict.set_item("ask_ct", self.levels[0].ask_ct).unwrap();
        dict.into()
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
    #[getter]
    fn price(&self) -> i64 {
        self.close
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py); // Correct usage of PyDict::new
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("open", self.open).unwrap();
        dict.set_item("high", self.high).unwrap();
        dict.set_item("low", self.low).unwrap();
        dict.set_item("close", self.close).unwrap();
        dict.set_item("volume", self.volume).unwrap();
        dict.into()
    }
}
