use mbinary::decode::Decoder;
use mbinary::encode::CombinedEncoder;
use mbinary::enums::{Dataset, Schema};
use mbinary::metadata::Metadata;
use mbinary::record_enum::RecordEnum;
use mbinary::record_ref::RecordRef;
use mbinary::records::{BidAskPair, Mbp1Msg, RecordHeader};
use mbinary::symbols::SymbolMap;
use std::io::Cursor;

#[test]
fn test_integration_test() -> anyhow::Result<()> {
    // Metadata
    let mut symbol_map = SymbolMap::new();
    symbol_map.add_instrument("AAPL", 1);
    symbol_map.add_instrument("TSLA", 2);

    let metadata = Metadata::new(
        Schema::Mbp1,
        Dataset::Equities,
        1234567898765,
        123456765432,
        symbol_map,
    );

    // Records
    let record1 = Mbp1Msg {
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

    let record2 = Mbp1Msg {
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
        discriminator: 1,
        levels: [BidAskPair {
            bid_px: 1,
            ask_px: 2,
            bid_sz: 2,
            ask_sz: 2,
            bid_ct: 1,
            ask_ct: 3,
        }],
    };

    let record_ref1: RecordRef = (&record1).into();
    let record_ref2: RecordRef = (&record2).into();
    let records = &[record_ref1, record_ref2];

    let mut buffer = Vec::new();

    // Encode
    let mut encoder = CombinedEncoder::new(&mut buffer);
    encoder
        .encode(&metadata, records)
        .expect("Error on encoding");

    // Test
    let cursor = Cursor::new(buffer);
    let mut decoder = Decoder::new(cursor)?;
    let decoded = decoder.decode().expect("Error decoding metadata.");

    // Validate
    // assert_eq!(decoded.0.unwrap(), metadata);
    assert_eq!(
        decoded,
        [RecordEnum::Mbp1(record1), RecordEnum::Mbp1(record2)]
    );
    Ok(())
}
