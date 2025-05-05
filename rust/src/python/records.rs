use crate::enums::{Action, RType, Side};
use crate::records::{BboMsg, BidAskPair, Mbp1Msg, OhlcvMsg, Record, RecordHeader, TradeMsg};
use crate::PRICE_SCALE;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::{PyAny, PyAnyMethods};

#[cfg_attr(feature = "python", pyclass(dict, module = "mbinary"))]
pub struct RecordMsg;

#[pymethods]
impl RecordMsg {
    #[staticmethod]
    fn is_record(py: Python, obj: &Bound<'_, PyAny>) -> bool {
        // Get the types of the custom Python classes
        let mbp1_type = &py.get_type::<Mbp1Msg>();
        let trade_type = &py.get_type::<TradeMsg>();
        let ohlcv_type = &py.get_type::<OhlcvMsg>();
        let bbo_type = &py.get_type::<BboMsg>();

        // Check if the object is an instance of any of the custom types
        obj.is_exact_instance(mbp1_type)
            || obj.is_exact_instance(trade_type)
            || obj.is_exact_instance(ohlcv_type)
            || obj.is_exact_instance(bbo_type)
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

    #[getter]
    fn pretty_bid_px(&self) -> f64 {
        self.bid_px as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_ask_px(&self) -> f64 {
        self.ask_px as f64 / PRICE_SCALE as f64
    }
}

#[pymethods]
impl Mbp1Msg {
    #[new]
    fn py_new(
        instrument_id: u32,
        ts_event: u64,
        rollover_flag: u8,
        price: i64,
        size: u32,
        action: Action,
        side: Side,
        flags: u8,
        depth: u8,
        ts_recv: u64,
        ts_in_delta: i32,
        sequence: u32,
        discriminator: u32,
        levels: [BidAskPair; 1],
    ) -> Self {
        Mbp1Msg {
            hd: RecordHeader::new::<Self>(instrument_id, ts_event, rollover_flag),
            price,
            size,
            action: action.into(),
            side: side.into(),
            flags,
            depth,
            ts_recv,
            ts_in_delta,
            sequence,
            discriminator,
            levels,
        }
    }

    #[setter]
    fn set_instrument_id(&mut self, instrument_id: u32) {
        self.hd.instrument_id = instrument_id;
    }

    #[getter]
    fn instrument_id(&self) -> u32 {
        self.hd.instrument_id
    }

    #[getter]
    fn ts(&self) -> u64 {
        self.timestamp()
    }

    #[getter]
    fn ts_event(&self) -> u64 {
        self.hd.ts_event
    }

    #[getter]
    fn rollover_flag(&self) -> u8 {
        self.hd.rollover_flag
    }

    #[getter]
    fn rtype(&self) -> RType {
        self.hd.rtype()
    }

    #[getter]
    fn pretty_price(&self) -> f64 {
        self.price as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_action(&self) -> Action {
        Action::try_from(self.action as u8).unwrap()
    }

    #[getter]
    fn pretty_side(&self) -> Side {
        Side::try_from(self.side as u8).unwrap()
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("rollover_flag", self.hd.rollover_flag)
            .unwrap();
        dict.set_item("price", self.price).unwrap();
        dict.set_item("size", self.size).unwrap();
        dict.set_item("action", self.action).unwrap();
        dict.set_item("side", self.side).unwrap();
        dict.set_item("flags", self.flags).unwrap();
        dict.set_item("depth", self.depth).unwrap();
        dict.set_item("ts_recv", self.ts_recv).unwrap();
        dict.set_item("ts_in_delta", self.ts_in_delta).unwrap();
        dict.set_item("sequence", self.sequence).unwrap();
        dict.set_item("discriminator", self.discriminator).unwrap();
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
impl TradeMsg {
    #[new]
    fn py_new(
        instrument_id: u32,
        ts_event: u64,
        rollover_flag: u8,
        price: i64,
        size: u32,
        action: Action,
        side: Side,
        flags: u8,
        depth: u8,
        ts_recv: u64,
        ts_in_delta: i32,
        sequence: u32,
    ) -> Self {
        TradeMsg {
            hd: RecordHeader::new::<Self>(instrument_id, ts_event, rollover_flag),
            price,
            size,
            action: action.into(),
            side: side.into(),
            flags,
            depth,
            ts_recv,
            ts_in_delta,
            sequence,
        }
    }

    // Allows for over ride of insturment id incase, not using the midas instrument_id
    #[setter]
    fn set_instrument_id(&mut self, instrument_id: u32) {
        self.hd.instrument_id = instrument_id;
    }

    #[getter]
    fn instrument_id(&self) -> u32 {
        self.hd.instrument_id
    }

    #[getter]
    fn ts(&self) -> u64 {
        self.timestamp()
    }

    #[getter]
    fn ts_event(&self) -> u64 {
        self.hd.ts_event
    }

    #[getter]
    fn rollover_flag(&self) -> u8 {
        self.hd.rollover_flag
    }

    #[getter]
    fn rtype(&self) -> RType {
        self.hd.rtype()
    }

    #[getter]
    fn pretty_price(&self) -> f64 {
        self.price as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_action(&self) -> Action {
        Action::try_from(self.action as u8).unwrap()
    }

    #[getter]
    fn pretty_side(&self) -> Side {
        Side::try_from(self.side as u8).unwrap()
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("rollover_flag", self.hd.rollover_flag)
            .unwrap();
        dict.set_item("price", self.price).unwrap();
        dict.set_item("size", self.size).unwrap();
        dict.set_item("action", self.action).unwrap();
        dict.set_item("side", self.side).unwrap();
        dict.set_item("flags", self.flags).unwrap();
        dict.set_item("depth", self.depth).unwrap();
        dict.set_item("ts_recv", self.ts_recv).unwrap();
        dict.set_item("ts_in_delta", self.ts_in_delta).unwrap();
        dict.set_item("sequence", self.sequence).unwrap();
        dict.into()
    }
}

#[pymethods]
impl BboMsg {
    #[new]
    fn py_new(
        instrument_id: u32,
        ts_event: u64,
        rollover_flag: u8,
        levels: [BidAskPair; 1],
    ) -> Self {
        BboMsg {
            hd: RecordHeader::new::<Self>(instrument_id, ts_event, rollover_flag),
            levels,
        }
    }

    #[setter]
    fn set_instrument_id(&mut self, instrument_id: u32) {
        self.hd.instrument_id = instrument_id;
    }

    #[setter]
    fn set_ts_event(&mut self, ts_event: u64) {
        self.hd.ts_event = ts_event;
    }

    #[getter]
    fn ts(&self) -> u64 {
        self.timestamp()
    }

    #[getter]
    fn rollover_flag(&self) -> u8 {
        self.hd.rollover_flag
    }

    // Need b/c of differnce in how rust and python handle fixed legnth arrays
    #[setter]
    fn set_bid_px(&mut self, bid_px: i64) {
        self.levels[0].bid_px = bid_px;
    }

    #[setter]
    fn set_ask_px(&mut self, ask_px: i64) {
        self.levels[0].ask_px = ask_px;
    }

    #[setter]
    fn set_bid_sz(&mut self, bid_sz: u32) {
        self.levels[0].bid_sz = bid_sz;
    }

    #[setter]
    fn set_ask_sz(&mut self, ask_sz: u32) {
        self.levels[0].ask_sz = ask_sz;
    }

    #[setter]
    fn set_bid_ct(&mut self, bid_ct: u32) {
        self.levels[0].bid_ct = bid_ct;
    }

    #[setter]
    fn set_ask_ct(&mut self, ask_ct: u32) {
        self.levels[0].ask_ct = ask_ct;
    }

    #[getter]
    fn instrument_id(&self) -> u32 {
        self.hd.instrument_id
    }

    #[getter]
    fn ts_event(&self) -> u64 {
        self.hd.ts_event
    }

    #[getter]
    fn rtype(&self) -> RType {
        self.hd.rtype()
    }

    #[getter]
    fn pretty_price(&self) -> f64 {
        self.levels[0].pretty_mid_px()
    }

    #[getter]
    fn price(&self) -> i64 {
        self.levels[0].mid_px()
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("rollover_flag", self.hd.rollover_flag)
            .unwrap();
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
    fn py_new(
        instrument_id: u32,
        ts_event: u64,
        rollover_flag: u8,
        open: i64,
        high: i64,
        low: i64,
        close: i64,
        volume: u64,
    ) -> Self {
        OhlcvMsg {
            hd: RecordHeader::new::<Self>(instrument_id, ts_event, rollover_flag),
            open,
            high,
            low,
            close,
            volume,
        }
    }

    #[setter]
    fn set_instrument_id(&mut self, instrument_id: u32) {
        self.hd.instrument_id = instrument_id;
    }

    #[getter]
    fn instrument_id(&self) -> u32 {
        self.hd.instrument_id
    }

    #[getter]
    fn ts(&self) -> u64 {
        self.timestamp()
    }

    #[getter]
    fn ts_event(&self) -> u64 {
        self.hd.ts_event
    }

    #[getter]
    fn rollover_flag(&self) -> u8 {
        self.hd.rollover_flag
    }

    #[getter]
    fn rtype(&self) -> RType {
        self.hd.rtype()
    }

    #[getter]
    fn pretty_open(&self) -> f64 {
        self.open as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_close(&self) -> f64 {
        self.close as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_high(&self) -> f64 {
        self.high as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_low(&self) -> f64 {
        self.low as f64 / PRICE_SCALE as f64
    }

    #[getter]
    fn pretty_price(&self) -> f64 {
        self.close as f64 / PRICE_SCALE as f64
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
    fn __dict__(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py); // Correct usage of PyDict::new
        dict.set_item("length", self.hd.length).unwrap();
        dict.set_item("rtype", self.hd.rtype).unwrap();
        dict.set_item("instrument_id", self.hd.instrument_id)
            .unwrap();
        dict.set_item("ts_event", self.hd.ts_event).unwrap();
        dict.set_item("rollover_flag", self.hd.rollover_flag)
            .unwrap();
        dict.set_item("open", self.open).unwrap();
        dict.set_item("high", self.high).unwrap();
        dict.set_item("low", self.low).unwrap();
        dict.set_item("close", self.close).unwrap();
        dict.set_item("volume", self.volume).unwrap();
        dict.into()
    }
}
