use mbn::decode::CombinedDecoder;
use mbn::encode::CombinedEncoder;
use mbn::enums::Schema;
use mbn::metadata::Metadata;
use mbn::record_enum::RecordEnum;
use mbn::record_ref::RecordRef;
use mbn::records::{BidAskPair, Mbp1Msg, RecordHeader};
use mbn::symbols::SymbolMap;
use std::io::Cursor;

#[test]
fn test_integration_test() {
    // Metadata
    let mut symbol_map = SymbolMap::new();
    symbol_map.add_instrument("AAPL", 1);
    symbol_map.add_instrument("TSLA", 2);

    let metadata = Metadata::new(Schema::Mbp1, 1234567898765, 123456765432, symbol_map);

    // Record
    // let ohlcv_msg1 = OhlcvMsg {
    //     hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
    //     open: 100,
    //     high: 200,
    //     low: 50,
    //     close: 150,
    //     volume: 1000,
    // };
    let record1 = Mbp1Msg {
        hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
        price: 1000,
        size: 10,
        action: 1,
        side: 1,
        depth: 0,
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

    let record2 = Mbp1Msg {
        hd: RecordHeader::new::<Mbp1Msg>(1, 1622471124),
        price: 1000,
        size: 10,
        action: 1,
        side: 1,
        depth: 0,
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
    // let ohlcv_msg2 = OhlcvMsg {
    //     hd: RecordHeader::new::<OhlcvMsg>(2, 1622471125),
    //     open: 110,
    //     high: 210,
    //     low: 55,
    //     close: 155,
    //     volume: 1100,
    // };

    let record_ref1: RecordRef = (&record1).into();
    let record_ref2: RecordRef = (&record2).into();
    let records = &[record_ref1, record_ref2];

    let mut buffer = Vec::new();
    let mut encoder = CombinedEncoder::new(&mut buffer);
    encoder
        .encode_metadata_and_records(&metadata, records)
        .expect("Error on encoding");

    println!("{:?}", buffer);
    // let binary_string: String = buffer
    //     .iter()
    //     .map(|byte| format!("\\x{:02x}", byte))
    //     .collect();

    // println!("{}", binary_string);

    // Test
    let cursor = Cursor::new(buffer);
    let mut decoder = CombinedDecoder::new(cursor);
    let decoded = decoder
        .decode_metadata_and_records()
        .expect("Error decoding metadata.");

    // Validate
    println!("{:?}", decoded);
    assert_eq!(decoded.0.unwrap(), metadata);
    assert_eq!(
        decoded.1,
        [RecordEnum::Mbp1(record1), RecordEnum::Mbp1(record2)]
    );
}
