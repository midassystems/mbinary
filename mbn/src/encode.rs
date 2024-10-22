use crate::metadata::Metadata;
use crate::record_ref::*;
use crate::METADATA_LENGTH;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

pub struct CombinedEncoder<W> {
    writer: W,
}

impl<W: Write> CombinedEncoder<W> {
    pub fn new(writer: W) -> Self {
        CombinedEncoder { writer }
    }

    pub fn encode_metadata(&mut self, metadata: &Metadata) -> io::Result<()> {
        let mut metadata_encoder = MetadataEncoder::new(&mut self.writer);
        metadata_encoder.encode_metadata(metadata)
    }

    pub fn encode_record(&mut self, record: &RecordRef) -> io::Result<()> {
        let mut record_encoder = RecordEncoder::new(&mut self.writer);
        record_encoder.encode_record(record)
    }

    pub fn encode_records(&mut self, records: &[RecordRef]) -> io::Result<()> {
        let mut record_encoder = RecordEncoder::new(&mut self.writer);
        record_encoder.encode_records(records)
    }

    pub fn encode(&mut self, metadata: &Metadata, records: &[RecordRef]) -> io::Result<()> {
        self.encode_metadata(metadata)?;
        self.encode_records(records)?;
        Ok(())
    }
}
pub struct MetadataEncoder<W> {
    writer: W,
    buffer: Vec<u8>,
}

impl<W: Write> MetadataEncoder<W> {
    pub fn new(writer: W) -> Self {
        MetadataEncoder {
            writer,
            buffer: vec![0; METADATA_LENGTH], // Initialize buffer with fixed size
        }
    }

    pub fn encode_metadata(&mut self, metadata: &Metadata) -> io::Result<()> {
        let serialized = metadata.serialize();
        self.buffer[..serialized.len()].copy_from_slice(&serialized);
        self.writer.write_all(&self.buffer)?;
        self.writer.flush()?;
        Ok(())
    }
}

pub struct RecordEncoder<W> {
    writer: W,
}

impl<W: Write> RecordEncoder<W> {
    pub fn new(writer: W) -> Self {
        RecordEncoder { writer }
    }

    pub fn encode_record(&mut self, record: &RecordRef) -> io::Result<()> {
        let bytes = record.as_ref();
        self.writer.write_all(bytes)?;
        Ok(())
    }

    pub fn encode_records(&mut self, records: &[RecordRef]) -> io::Result<()> {
        for record in records {
            self.encode_record(record)?;
        }
        self.writer.flush()?;
        Ok(())
    }

    pub fn write_to_file(&self, file_path: &Path) -> io::Result<()>
    where
        W: AsRef<[u8]>, // Ensure W can be treated as a slice of bytes (like Vec<u8>)
    {
        // Open the file in append mode, create it if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true) // Append to the file if it exists
            .open(file_path)?;

        file.write_all(self.writer.as_ref())?; // Write the internal buffer to file
        file.flush()?; // Ensure all data is flushed to disk
        Ok(())
    }

    // pub fn write_to_file(&self, file_path: &Path) -> io::Result<()>
    // where
    //     W: AsRef<[u8]>, // Ensure W can be treated as a slice of bytes (like Vec<u8>)
    // {
    //     let mut file = std::fs::File::create(file_path)?;
    //     file.write_all(self.writer.as_ref())?; // Write the internal buffer to file
    //     file.flush()?; // Ensure all data is flushed to disk
    //     Ok(())
    // }
}

// pub fn write_to_file(file_path: &Path, buffer: &[u8]) -> io::Result<()> {
//     let mut file = std::fs::File::create(file_path)?;
//     file.write_all(buffer)?;
//     file.flush()?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::RecordDecoder;
    use crate::enums::Schema;
    use crate::record_enum::RecordEnum;
    use crate::records::OhlcvMsg;
    use crate::records::RecordHeader;
    use crate::symbols::SymbolMap;
    use std::io::Cursor;

    #[test]
    fn test_encode_record() {
        let ohlcv_msg = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100,
            high: 200,
            low: 50,
            close: 150,
            volume: 1000,
        };
        let record_ref: RecordRef = (&ohlcv_msg).into();

        // Test
        let mut buffer = Vec::new();
        let mut encoder = RecordEncoder::new(&mut buffer);
        encoder.encode_record(&record_ref).expect("Encoding failed");

        // Validate
        let cursor = Cursor::new(buffer);
        let mut decoder = RecordDecoder::new(cursor);
        let record_ref = decoder.decode_ref().unwrap().unwrap();
        let decoded_record: &OhlcvMsg = record_ref.get().unwrap();
        assert_eq!(decoded_record, &ohlcv_msg);
    }

    #[test]
    fn test_encode_decode_records() {
        let ohlcv_msg1 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1622471124),
            open: 100000000000,
            high: 200000000000,
            low: 50000000000,
            close: 150000000000,
            volume: 1000,
        };

        let ohlcv_msg2 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 1622471125),
            open: 110000000000,
            high: 210000000000,
            low: 55000000000,
            close: 155000000000,
            volume: 1100,
        };

        let record_ref1: RecordRef = (&ohlcv_msg1).into();
        let record_ref2: RecordRef = (&ohlcv_msg2).into();

        // Test
        let mut buffer = Vec::new();
        let mut encoder = RecordEncoder::new(&mut buffer);
        encoder
            .encode_records(&[record_ref1, record_ref2])
            .expect("Encoding failed");
        // println!("{:?}", buffer);

        // Validate
        let cursor = Cursor::new(buffer);
        let mut decoder = RecordDecoder::new(cursor);
        let decoded_records = decoder.decode_to_owned().expect("Decoding failed");

        assert_eq!(decoded_records.len(), 2);
        assert_eq!(decoded_records[0], RecordEnum::Ohlcv(ohlcv_msg1));
        assert_eq!(decoded_records[1], RecordEnum::Ohlcv(ohlcv_msg2));
    }

    #[test]
    fn test_encode_metadata() {
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument("AAPL", 1);
        symbol_map.add_instrument("TSLA", 2);

        let metadata = Metadata::new(Schema::Ohlcv1S, 1234567898765, 123456765432, symbol_map);

        // Test
        let mut buffer = Vec::new();
        let mut encoder = MetadataEncoder::new(&mut buffer);
        encoder
            .encode_metadata(&metadata)
            .expect("Error metadata encoding.");

        // Validate
        let decoded = Metadata::deserialize(&buffer);
        assert_eq!(decoded.schema, metadata.schema);
        assert_eq!(decoded.start, metadata.start);
        assert_eq!(decoded.end, metadata.end);
        assert_eq!(decoded.mappings, metadata.mappings);
    }

    #[test]
    fn test_encode() {
        // Metadata
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument("AAPL", 1);
        symbol_map.add_instrument("TSLA", 2);

        let metadata = Metadata::new(Schema::Ohlcv1S, 1234567898765, 123456765432, symbol_map);

        // Record
        let ohlcv_msg1 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(1, 1724287878000000000),
            open: 100000000000,
            high: 200000000000,
            low: 50000000000,
            close: 150000000000,
            volume: 1000000000000,
        };

        let ohlcv_msg2 = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 1724289878000000000),
            open: 110000000000,
            high: 210000000000,
            low: 55000000000,
            close: 155000000000,
            volume: 1100000000000,
        };

        let record_ref1: RecordRef = (&ohlcv_msg1).into();
        let record_ref2: RecordRef = (&ohlcv_msg2).into();
        let records = &[record_ref1, record_ref2];

        // Test
        let mut buffer = Vec::new();
        let mut encoder = CombinedEncoder::new(&mut buffer);
        encoder
            .encode(&metadata, records)
            .expect("Error on encoding");

        // Validate
        assert!(buffer.len() > 0);
    }
}
