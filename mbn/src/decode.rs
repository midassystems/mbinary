use crate::decode_iterator::DecoderIterator;
use crate::metadata::Metadata;
use crate::record_enum::RecordEnum;
use crate::record_ref::*;
use crate::records::RecordHeader;
use std::io::{self, Cursor, Read};
use std::mem;

use crate::METADATA_LENGTH; // Import the constant

pub struct CombinedDecoder<R> {
    reader: R,
}

impl<R: Read> CombinedDecoder<R> {
    pub fn new(reader: R) -> Self {
        CombinedDecoder { reader }
    }

    pub fn decode_metadata(&mut self) -> io::Result<Option<Metadata>> {
        let mut metadata_decoder = MetadataDecoder::new(&mut self.reader);
        metadata_decoder.decode()
    }

    pub fn decode_all_records(&mut self) -> io::Result<Vec<RecordEnum>> {
        let mut record_decoder = RecordDecoder::new(&mut self.reader);
        record_decoder.decode_to_owned()
    }

    pub fn decode_iterator(&mut self) -> DecoderIterator<R> {
        DecoderIterator::new(&mut self.reader)
    }

    pub fn decode_metadata_and_records(
        &mut self,
    ) -> io::Result<(Option<Metadata>, Vec<RecordEnum>)> {
        let metadata = self.decode_metadata()?;
        let records = self.decode_all_records()?;
        Ok((metadata, records))
    }
}

pub struct MetadataDecoder<R> {
    reader: R,
    read_buffer: Vec<u8>,
}

impl<R: Read> MetadataDecoder<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            read_buffer: vec![0; METADATA_LENGTH], // Initialize buffer with fixed size
        }
    }

    pub fn decode(&mut self) -> io::Result<Option<Metadata>> {
        self.reader.read_exact(&mut self.read_buffer)?;

        let metadata = Metadata::deserialize(&self.read_buffer);
        Ok(Some(metadata))
    }
}

pub struct RecordDecoder<R> {
    reader: R,
    read_buffer: Vec<u8>,
}

impl<R> RecordDecoder<R>
where
    R: Read,
{
    /// Creates a new `RecordDecoder` that will decode from `reader`.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            read_buffer: vec![0],
        }
    }

    pub fn decode_to_owned(&mut self) -> io::Result<Vec<RecordEnum>> {
        let mut records = Vec::new();
        while let Some(record_ref) = self.decode_ref()? {
            if let Some(record) = RecordEnum::from_ref(record_ref) {
                records.push(record);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "record with rtype {:?} could not be converted to the target type",
                        record_ref.header().rtype()
                    ),
                ));
            }
        }
        Ok(records)
    }

    pub fn decode_iterator(&mut self) -> DecoderIterator<R> {
        DecoderIterator::new(&mut self.reader)
    }

    pub fn decode_ref(&mut self) -> io::Result<Option<RecordRef>> {
        if let Err(err) = self.reader.read_exact(&mut self.read_buffer[..1]) {
            if err.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(None);
            } else {
                return Err(io::Error::new(
                    err.kind(),
                    format!("decoding record reference: {}", err),
                ));
            }
        }
        let length = self.read_buffer[0] as usize * RecordHeader::LENGTH_MULTIPLIER;
        if length < mem::size_of::<RecordHeader>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid record with length {} shorter than header", length),
            ));
        }
        if length > self.read_buffer.len() {
            self.read_buffer.resize(length, 0);
        }
        if let Err(err) = self.reader.read_exact(&mut self.read_buffer[1..length]) {
            if err.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(None);
            } else {
                return Err(io::Error::new(
                    err.kind(),
                    format!("decoding record reference: {}", err),
                ));
            }
        }
        // Safety: `read_buffer` is resized to contain at least `length` bytes.
        Ok(Some(unsafe { RecordRef::new(&self.read_buffer) }))
    }
}

pub fn decoder_from_file(file_path: &str) -> io::Result<RecordDecoder<Cursor<Vec<u8>>>> {
    // Open the file
    let mut file = std::fs::File::open(file_path)?;

    // Read the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create a Cursor from the buffer
    let cursor = Cursor::new(buffer);

    // Return the RecordDecoder using the cursor
    Ok(RecordDecoder::new(cursor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::MetadataEncoder;
    use crate::encode::{CombinedEncoder, RecordEncoder};
    use crate::enums::{RType, Schema};
    use crate::records::{as_u8_slice, OhlcvMsg};
    use crate::symbols::SymbolMap;
    use std::io::Cursor;

    #[test]
    fn test_decode_record() {
        // Create an OhlcvMsg record
        let ohlcv_msg = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };

        // Convert the record to a byte slice
        let data = unsafe { as_u8_slice(&ohlcv_msg) };

        // Create a cursor from the byte slice
        let cursor = Cursor::new(data.to_vec());

        // Initialize the RecordDecoder with the cursor
        let mut decoder = RecordDecoder::new(cursor);

        // Decode the record
        let result = decoder.decode_to_owned().expect("Error decoding.");

        // Verify the result
        if let Some(RecordEnum::Ohlcv(msg)) = result.get(0) {
            assert_eq!(msg.open, 100);
            assert_eq!(msg.high, 200);
            assert_eq!(msg.low, 50);
            assert_eq!(msg.close, 150);
            assert_eq!(msg.volume, 1000);
        } else {
            panic!("Failed to decode record");
        }
    }

    #[test]
    fn test_decode_record_ref() {
        // Create an OhlcvMsg record
        let ohlcv_msg = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };

        // Convert the record to a byte slice
        let data = unsafe { as_u8_slice(&ohlcv_msg) };

        // Create a cursor from the byte slice
        let cursor = Cursor::new(data.to_vec());

        // Initialize the RecordDecoder with the cursor
        let mut decoder = RecordDecoder::new(cursor);

        // Decode the record reference
        let result = decoder.decode_ref();

        // Verify the result
        assert!(result.is_ok());
        if let Some(record_ref) = result.unwrap() {
            let header = record_ref.header();
            assert_eq!(header.instrument_id, 1);
            assert_eq!(header.ts_event, 1622471124);
            assert_eq!(header.rtype, RType::Ohlcv as u8);
        } else {
            panic!("Failed to decode record reference");
        }
    }

    #[test]
    fn test_encode_decode_records() {
        let ohlcv_msg1 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };

        let ohlcv_msg2 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 1622471125),
            open: 110,
            high: 210,
            low: 55,
            close: 155,
            volume: 1100,
        };

        let mut buffer = Vec::new();
        {
            let mut encoder = RecordEncoder::new(&mut buffer);
            let record_ref1: RecordRef = (&ohlcv_msg1).into();
            let record_ref2: RecordRef = (&ohlcv_msg2).into();
            encoder
                .encode_records(&[record_ref1, record_ref2])
                .expect("Encoding failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = RecordDecoder::new(cursor);

        let decoded_records: Vec<RecordEnum> = decoder.decode_to_owned().expect("Decoding failed");

        assert_eq!(decoded_records.len(), 2);
        assert_eq!(decoded_records[0], RecordEnum::Ohlcv(ohlcv_msg1));
        assert_eq!(decoded_records[1], RecordEnum::Ohlcv(ohlcv_msg2));
    }

    #[test]
    fn test_decode_metadata() {
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument("AAPL", 1);
        symbol_map.add_instrument("TSLA", 2);

        let metadata = Metadata::new(Schema::Ohlcv1S, 1234567898765, 123456765432, symbol_map);

        let mut buffer = Vec::new();
        let mut encoder = MetadataEncoder::new(&mut buffer);
        encoder
            .encode_metadata(&metadata)
            .expect("Error metadata encoding.");

        // Test
        let cursor = Cursor::new(buffer);
        let mut decoder = MetadataDecoder::new(cursor);
        let decoded = decoder.decode().expect("Error decoding metadata.").unwrap();

        // Validate
        assert_eq!(decoded.schema, metadata.schema);
        assert_eq!(decoded.start, metadata.start);
        assert_eq!(decoded.end, metadata.end);
        assert_eq!(decoded.mappings, metadata.mappings);
    }

    #[test]
    fn test_decode() {
        // Metadata
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument("AAPL", 1);
        symbol_map.add_instrument("TSLA", 2);

        let metadata = Metadata::new(Schema::Ohlcv1S, 1234567898765, 123456765432, symbol_map);

        // Record
        let ohlcv_msg1 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };

        let ohlcv_msg2 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 1622471125),
            open: 110,
            high: 210,
            low: 55,
            close: 155,
            volume: 1100,
        };

        let record_ref1: RecordRef = (&ohlcv_msg1).into();
        let record_ref2: RecordRef = (&ohlcv_msg2).into();
        let records = &[record_ref1, record_ref2];

        let mut buffer = Vec::new();
        let mut encoder = CombinedEncoder::new(&mut buffer);
        encoder
            .encode_metadata_and_records(&metadata, records)
            .expect("Error on encoding");

        // Test
        let cursor = Cursor::new(buffer);
        let mut decoder = CombinedDecoder::new(cursor);
        let decoded = decoder
            .decode_metadata_and_records()
            .expect("Error decoding metadata.");

        // Validate
        assert_eq!(decoded.0.unwrap(), metadata);
        assert_eq!(
            decoded.1,
            [RecordEnum::Ohlcv(ohlcv_msg1), RecordEnum::Ohlcv(ohlcv_msg2)]
        );
    }

    #[test]
    fn test_iter_decode() {
        // Setup
        let ohlcv_msg1 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };

        let ohlcv_msg2 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 1622471125),
            open: 110,
            high: 210,
            low: 55,
            close: 155,
            volume: 1100,
        };

        // Encode
        let mut buffer = Vec::new();
        {
            let mut encoder = RecordEncoder::new(&mut buffer);
            let record_ref1: RecordRef = (&ohlcv_msg1).into();
            let record_ref2: RecordRef = (&ohlcv_msg2).into();
            encoder
                .encode_records(&[record_ref1, record_ref2])
                .expect("Encoding failed");
        }

        // Decode
        let cursor = Cursor::new(buffer);
        let mut decoder = RecordDecoder::new(cursor);
        let iter = decoder.decode_iterator();

        // Test
        let mut i = 0;
        for record in iter {
            match record {
                Ok(record) => {
                    // Process the record
                    if i == 0 {
                        assert_eq!(record, RecordEnum::Ohlcv(ohlcv_msg1.clone()));
                    } else {
                        assert_eq!(record, RecordEnum::Ohlcv(ohlcv_msg2.clone()));
                    }
                    i = i + 1;
                }
                Err(e) => {
                    eprintln!("Error processing record: {:?}", e);
                }
            }
        }
    }
}
