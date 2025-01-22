use crate::records::{HasRType, Record, RecordHeader};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

#[derive(Debug, Clone, Copy)]
pub struct RecordRef<'a> {
    ptr: NonNull<RecordHeader>,
    _marker: PhantomData<&'a RecordHeader>,
}
// Safety: RecordRef is safe to send and share across threads as long as the underlying data
// it points to is not mutated unsafely.
unsafe impl<'a> Send for RecordRef<'a> {}
unsafe impl<'a> Sync for RecordRef<'a> {}

impl<'a> RecordRef<'a> {
    pub unsafe fn new(buffer: &'a [u8]) -> Self {
        debug_assert!(buffer.len() >= std::mem::size_of::<RecordHeader>());
        let raw_ptr = buffer.as_ptr() as *mut RecordHeader;
        let ptr = NonNull::new_unchecked(raw_ptr);
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn header(&self) -> &'a RecordHeader {
        unsafe { self.ptr.as_ref() }
    }

    pub fn record_size(&self) -> usize {
        self.header().record_size()
    }

    pub fn has<T: HasRType>(&self) -> bool {
        T::has_rtype(self.header().rtype)
    }

    pub fn get<T: HasRType>(&self) -> Option<&'a T> {
        if self.has::<T>() {
            assert!(self.record_size() >= std::mem::size_of::<T>());
            Some(unsafe { self.ptr.cast::<T>().as_ref() })
        } else {
            None
        }
    }
}

impl<'a, R> From<&'a R> for RecordRef<'a>
where
    R: Record,
{
    fn from(rec: &'a R) -> Self {
        Self {
            ptr: unsafe {
                NonNull::new_unchecked((rec.header() as *const RecordHeader).cast_mut())
            },
            _marker: PhantomData,
        }
    }
}

impl<'a> AsRef<[u8]> for RecordRef<'a> {
    fn as_ref(&self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr() as *const u8, self.record_size()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::OhlcvMsg;

    #[test]
    fn test_record_ref() {
        let record = OhlcvMsg {
            hd: RecordHeader::new::<OhlcvMsg>(2, 162222293489348, 0),
            open: 909,
            high: 11991,
            low: 800,
            close: 999,
            volume: 123456765432,
        };

        // Test
        let record_ref = RecordRef::from(&record);
        let bytes = record_ref.as_ref();

        // Validate
        let new_ref = unsafe { RecordRef::new(bytes) };
        let decoded_record: &OhlcvMsg = new_ref.get().unwrap();
        assert_eq!(&record, decoded_record);
    }
}
