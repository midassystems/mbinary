use crate::enums::RType;
use crate::record_ref::RecordRef;
use crate::records::{Mbp1Msg, OhlcvMsg, Record, RecordHeader};
use serde::Serialize;

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
}
impl AsRef<[u8]> for RecordEnum {
    fn as_ref(&self) -> &[u8] {
        match self {
            RecordEnum::Mbp1(msg) => msg.as_ref(),
            RecordEnum::Ohlcv(msg) => msg.as_ref(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::BidAskPair;

    #[test]
    fn test_encode_decode_record_enum() {
        let record_enum = RecordEnum::Mbp1(Mbp1Msg {
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
        });

        // Test
        let record_ref = record_enum.to_record_ref();
        let bytes = record_ref.as_ref();
        let new_ref = unsafe { RecordRef::new(bytes) };
        let ref_enum = RecordEnumRef::from_ref(new_ref).unwrap();
        let decoded = ref_enum.to_owned();

        // Validate
        assert_eq!(decoded, record_enum);
    }
}
