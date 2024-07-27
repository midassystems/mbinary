use crate::records::{Mbp1Msg, OhlcvMsg, RecordHeader};
use pyo3::prelude::*;

// TODO: Add BIDAsK PAIR

// #[pymethods]
// impl Mbp1Msg {
//     #[new]
//     fn py_new(
//         hd: RecordHeader,
//         price: i64,
//         size: u32,
//         action: u16,
//         side: u16,
//         flags: u8,
//         ts_recv: u64,
//         ts_in_delta: i32,
//         sequence: u32,
//     ) -> Self {
//         Mbp1Msg {
//             hd,
//             price,
//             size,
//             action,
//             side,
//             flags,
//             ts_recv,
//             ts_in_delta,
//             sequence,
//         }
//     }

//     fn __str__(&self) -> String {
//         format!("{:?}", self)
//     }
// }

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
