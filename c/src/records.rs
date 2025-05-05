use mbinary::enums::RType;
use mbinary::record_enum::RecordEnum;
use mbinary::record_ref::RecordRef;
use mbinary::records::{BboMsg, Mbp1Msg, OhlcvMsg, Record, RecordHeader, TradeMsg};

// Gives polymorphic behaviour to the CRecordEnum.data field.
#[repr(C)]
pub union RecordData {
    mbp1: Mbp1Msg,
    ohlcv: OhlcvMsg,
    trade: TradeMsg,
    tbbo: Mbp1Msg,
    bbo: BboMsg,
}

impl RecordData {
    pub fn to_record_ref(&self) -> RecordRef<'_> {
        return unsafe { RecordRef::from(&self.mbp1) };
    }
}

#[repr(C)]
pub struct CRecordEnum {
    pub rtype: RType,
    pub data: RecordData,
}

// Non-public implmentations more for tests being able to encode and decode in place
#[allow(unused)]
impl CRecordEnum {
    fn record_ref(&self) -> RecordRef<'_> {
        match &self.rtype {
            RType::Mbp1 => return unsafe { RecordRef::from(&self.data.mbp1) },
            RType::Tbbo => return unsafe { RecordRef::from(&self.data.mbp1) },
            RType::Trades => return unsafe { RecordRef::from(&self.data.trade) },
            RType::Ohlcv => return unsafe { RecordRef::from(&self.data.ohlcv) },
            RType::Bbo => return unsafe { RecordRef::from(&self.data.bbo) },
        }
    }
}

impl From<RecordEnum> for CRecordEnum {
    fn from(value: RecordEnum) -> Self {
        match value {
            RecordEnum::Mbp1(msg) => CRecordEnum {
                rtype: RType::Mbp1,
                data: RecordData { mbp1: msg },
            },
            RecordEnum::Tbbo(msg) => CRecordEnum {
                rtype: RType::Tbbo,
                data: RecordData { tbbo: msg },
            },
            RecordEnum::Trade(msg) => CRecordEnum {
                rtype: RType::Trades,
                data: RecordData { trade: msg },
            },
            RecordEnum::Ohlcv(msg) => CRecordEnum {
                rtype: RType::Ohlcv,
                data: RecordData { ohlcv: msg },
            },
            RecordEnum::Bbo(msg) => CRecordEnum {
                rtype: RType::Bbo,
                data: RecordData { bbo: msg },
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn output(record: *const CRecordEnum) {
    unsafe {
        match &(*record).rtype {
            RType::Mbp1 => println!("{:?}", (*record).data.mbp1),
            RType::Tbbo => println!("{:?}", (*record).data.tbbo),
            RType::Trades => println!("{:?}", (*record).data.trade),
            RType::Ohlcv => println!("{:?}", (*record).data.ohlcv),
            RType::Bbo => println!("{:?}", (*record).data.bbo),
        }
    }
}

#[no_mangle]
pub extern "C" fn get_header(record: *const CRecordEnum) -> *const RecordHeader {
    unsafe {
        match &(*record).rtype {
            RType::Mbp1 => return (*record).data.mbp1.header(),
            RType::Tbbo => return (*record).data.tbbo.header(),
            RType::Trades => return (*record).data.trade.header(),
            RType::Ohlcv => return (*record).data.ohlcv.header(),
            RType::Bbo => return (*record).data.bbo.header(),
        }
    }
}

#[no_mangle]
pub extern "C" fn get_timestamp(record: *const CRecordEnum) -> u64 {
    unsafe {
        match &(*record).rtype {
            RType::Mbp1 => return (*record).data.mbp1.timestamp(),
            RType::Tbbo => return (*record).data.tbbo.timestamp(),
            RType::Trades => return (*record).data.trade.timestamp(),
            RType::Ohlcv => return (*record).data.ohlcv.timestamp(),
            RType::Bbo => return (*record).data.bbo.timestamp(),
        }
    }
}

#[no_mangle]
pub extern "C" fn get_price(record: *const CRecordEnum) -> i64 {
    unsafe {
        match &(*record).rtype {
            RType::Mbp1 => return (*record).data.mbp1.price(),
            RType::Tbbo => return (*record).data.tbbo.price(),
            RType::Trades => return (*record).data.trade.price(),
            RType::Ohlcv => return (*record).data.ohlcv.price(),
            RType::Bbo => return (*record).data.bbo.price(),
        }
    }
}
