use crate::enums::RType;
use crate::record_ref::RecordRef;
use crate::records::{Mbp1Msg, OhlcvMsg, Record, RecordHeader};
// use polars::prelude::*;
use serde::Serialize;
use std::collections::hash_map::HashMap;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RecordEnum {
    Mbp1(Mbp1Msg),
    Ohlcv(OhlcvMsg),
}

impl RecordEnum {
    pub fn from_ref(rec_ref: RecordRef) -> Option<Self> {
        match rec_ref.header().rtype() {
            RType::Mbp1 => rec_ref
                .get::<Mbp1Msg>()
                .map(|msg| RecordEnum::Mbp1(msg.clone())),
            RType::Ohlcv => rec_ref
                .get::<OhlcvMsg>()
                .map(|msg| RecordEnum::Ohlcv(msg.clone())),
        }
    }

    pub fn to_record_ref(&self) -> RecordRef {
        match self {
            RecordEnum::Mbp1(record) => record.into(),
            RecordEnum::Ohlcv(record) => record.into(),
            // Add cases for other record types if needed
        }
    }

    pub fn to_ref<'a>(&'a self) -> RecordEnumRef<'a> {
        match self {
            RecordEnum::Mbp1(msg) => RecordEnumRef::Mbp1(msg),
            RecordEnum::Ohlcv(msg) => RecordEnumRef::Ohlcv(msg),
        }
    }
    pub fn msg(&self) -> &dyn Record {
        match self {
            RecordEnum::Mbp1(msg) => msg as &dyn Record,
            RecordEnum::Ohlcv(msg) => msg as &dyn Record,
        }
    }

    // pub fn extract_fields(&self) -> HashMap<&'static str, Vec<AnyValue>> {
    //     let mut fields = HashMap::new();
    //     match self {
    //         RecordEnum::Mbp1(msg) => {
    //             fields.insert("price", vec![AnyValue::Int64(msg.price)]);
    //             fields.insert("size", vec![AnyValue::UInt32(msg.size)]);
    //             fields.insert("action", vec![AnyValue::UInt16(msg.action)]);
    //             fields.insert("side", vec![AnyValue::UInt16(msg.side)]);
    //             fields.insert("flags", vec![AnyValue::UInt8(msg.flags)]);
    //             fields.insert("ts_recv", vec![AnyValue::UInt64(msg.ts_recv)]);
    //             fields.insert("ts_in_delta", vec![AnyValue::Int32(msg.ts_in_delta)]);
    //             fields.insert("sequence", vec![AnyValue::UInt32(msg.sequence)]);
    //         }
    //         RecordEnum::Ohlcv(msg) => {
    //             fields.insert("open", vec![AnyValue::Int64(msg.open)]);
    //             fields.insert("high", vec![AnyValue::Int64(msg.high)]);
    //             fields.insert("low", vec![AnyValue::Int64(msg.low)]);
    //             fields.insert("close", vec![AnyValue::Int64(msg.close)]);
    //             fields.insert("volume", vec![AnyValue::UInt64(msg.volume)]);
    //         }
    //     }
    //     fields
    // }
}

impl Record for RecordEnum {
    fn header(&self) -> &RecordHeader {
        match self {
            RecordEnum::Mbp1(msg) => &msg.hd,
            RecordEnum::Ohlcv(msg) => &msg.hd,
        }
    }
}

#[cfg(feature = "python")]
impl IntoPy<Py<PyAny>> for RecordEnum {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            RecordEnum::Mbp1(msg) => msg.into_py(py).into(),
            RecordEnum::Ohlcv(msg) => msg.into_py(py).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RecordEnumRef<'a> {
    Mbp1(&'a Mbp1Msg),
    Ohlcv(&'a OhlcvMsg),
}

impl<'a> RecordEnumRef<'a> {
    pub fn from_ref(rec_ref: RecordRef<'a>) -> Option<Self> {
        match rec_ref.header().rtype() {
            RType::Mbp1 => rec_ref.get::<Mbp1Msg>().map(RecordEnumRef::Mbp1),
            RType::Ohlcv => rec_ref.get::<OhlcvMsg>().map(RecordEnumRef::Ohlcv),
        }
    }

    pub fn to_owned(&self) -> RecordEnum {
        match self {
            RecordEnumRef::Mbp1(msg) => RecordEnum::Mbp1((*msg).clone()),
            RecordEnumRef::Ohlcv(msg) => RecordEnum::Ohlcv((*msg).clone()),
        }
    }
}

impl<'a> Record for RecordEnumRef<'a> {
    fn header(&self) -> &RecordHeader {
        match self {
            RecordEnumRef::Mbp1(msg) => &msg.hd,
            RecordEnumRef::Ohlcv(msg) => &msg.hd,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::records::{
//         as_u8_slice, transmute_header_bytes, transmute_record, transmute_record_bytes,
//     };

//     #[test]
//     fn test_construction_record_enum() {
//         let record = RecordEnum::Mbp1(Mbp1Msg {
//             hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
//             price: 1000,
//             size: 10,
//             action: 1,
//             side: 1,
//             flags: 0,
//             ts_recv: 123456789098765,
//             ts_in_delta: 12345,
//             sequence: 123456,
//         });

//         // Test
//         let bytes = record.as_ref();

//         //
//         let decoded: Mbp1Msg = unsafe { transmute_record_bytes(bytes).unwrap() };

//         // Validate
//         assert_eq!(decoded, record);
//     }

//     #[test]
//     fn test_transmute_record_enum() {}
// }
