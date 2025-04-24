use mbinary::decode::RecordDecoder;
use std::ffi::CStr;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::ptr;
use std::slice;

use crate::records::CRecordEnum;

/// C-compatible wrapper around RecordDecoder
pub struct CRecordDecoder {
    decoder: RecordDecoder<Box<dyn Read>>,
}

/// Create a new `CRecordDecoder` with an in-memory buffer as the source.
#[no_mangle]
pub extern "C" fn create_buffer_decoder(
    source: *const u8,
    source_size: usize,
) -> *mut CRecordDecoder {
    if source.is_null() || source_size == 0 {
        return ptr::null_mut();
    }

    // Convert the raw pointer and size to a Vec<u8>
    let source_slice = unsafe { slice::from_raw_parts(source, source_size) };
    let source_buffer = Cursor::new(source_slice.to_vec());

    let decoder = CRecordDecoder {
        decoder: RecordDecoder::new(Box::new(source_buffer)),
    };

    Box::into_raw(Box::new(decoder))
}

/// Create a new `CRecordDecoder` with a file as the source.
#[no_mangle]
pub extern "C" fn create_file_decoder(file_path: *const libc::c_char) -> *mut CRecordDecoder {
    if file_path.is_null() {
        return ptr::null_mut();
    }

    // Convert C string to Rust Path
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let path = match c_str.to_str() {
        Ok(path) => path,
        Err(_) => return ptr::null_mut(),
    };

    let decoder = match create_decoder_from_file(path) {
        Ok(f) => f,
        Err(_) => return ptr::null_mut(),
    };

    let c_decoder = CRecordDecoder { decoder };

    Box::into_raw(Box::new(c_decoder))
}

// Helper function, not exposed to C directly.
fn create_decoder_from_file(
    file_path: &str,
) -> Result<RecordDecoder<Box<dyn Read>>, std::io::Error> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let boxed_reader: Box<dyn Read> = Box::new(buf_reader);
    Ok(RecordDecoder::new(boxed_reader))
}

/// Iteratively decodes records, returning false when all records are decoded.
#[no_mangle]
pub extern "C" fn decoder_iter(decoder: *mut CRecordDecoder, output: *mut CRecordEnum) -> bool {
    if decoder.is_null() || output.is_null() {
        return false;
    }

    let decoder = unsafe { &mut *decoder };
    let mut iterator = decoder.decoder.decode_iterator();

    match iterator.next() {
        Some(Ok(record_enum)) => {
            let c_record: CRecordEnum = record_enum.into();
            unsafe { ptr::write(output, c_record) };
            true
        }
        _ => false,
    }
}

/// Destroy the `CRecordDecoder`
#[no_mangle]
pub extern "C" fn destroy_record_decoder(decoder: *mut CRecordDecoder) {
    if decoder.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(decoder);
    }
}
