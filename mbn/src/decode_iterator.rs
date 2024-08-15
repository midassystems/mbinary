use crate::decode::RecordDecoder;
use crate::record_enum::RecordEnum;
use std::io::Read;

pub struct DecoderIterator<'a, R> {
    decoder: RecordDecoder<&'a mut R>,
}

impl<'a, R: Read> DecoderIterator<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            decoder: RecordDecoder::new(reader),
        }
    }
}

impl<'a, R: Read> Iterator for DecoderIterator<'a, R> {
    type Item = std::io::Result<RecordEnum>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.decoder.decode_ref() {
            Ok(Some(record_ref)) => match RecordEnum::from_ref(record_ref) {
                Some(record) => Some(Ok(record)),
                None => Some(Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to convert record reference to RecordEnum",
                ))),
            },
            Ok(None) => None,       // End of the iteration
            Err(e) => Some(Err(e)), // Propagate the error
        }
    }
}
