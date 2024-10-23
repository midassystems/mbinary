use crate::decode::{AsyncRecordDecoder, RecordDecoder};
use crate::record_enum::RecordEnum;
use futures::stream::Stream;
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncBufRead;

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
                Ok(record) => Some(Ok(record)),
                Err(_) => Some(Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to convert record reference to RecordEnum",
                ))),
            },
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct AsyncDecoderIterator<'a, R> {
    decoder: AsyncRecordDecoder<&'a mut R>,
}

impl<'a, R: AsyncBufRead + Unpin> AsyncDecoderIterator<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            decoder: AsyncRecordDecoder::new(reader),
        }
    }
}

impl<'a, R: AsyncBufRead + Unpin> Stream for AsyncDecoderIterator<'a, R> {
    type Item = std::io::Result<RecordEnum>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Poll for the next record asynchronously
        let fut = self.decoder.decode_ref();
        let mut fut = Box::pin(fut); // Pin the future

        match Future::poll(fut.as_mut(), cx) {
            Poll::Ready(Ok(Some(record_ref))) => {
                // If the record_ref is decoded successfully, convert it to RecordEnum
                match RecordEnum::from_ref(record_ref) {
                    Ok(record) => Poll::Ready(Some(Ok(record))),
                    Err(_) => Poll::Ready(Some(Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Failed to convert record reference to RecordEnum",
                    )))),
                }
            }
            Poll::Ready(Ok(None)) => Poll::Ready(None),
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
