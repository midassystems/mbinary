use mbinary::encode::RecordEncoder;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

use crate::records::RecordData;

/// C-compatible wrapper around RecordEncoder
pub struct CRecordEncoder {
    buffer: Vec<u8>,
}

/// Create a new 'CRecordEncoder', encoding to a buffer.
#[no_mangle]
pub extern "C" fn create_record_encoder() -> *mut CRecordEncoder {
    let c_encoder = CRecordEncoder { buffer: Vec::new() };
    Box::into_raw(Box::new(c_encoder))
}

#[no_mangle]
pub extern "C" fn encode_records(
    encoder: *mut CRecordEncoder,
    records: *const RecordData,
    record_count: usize,
) -> i32 {
    if encoder.is_null() || records.is_null() {
        return -1;
    }

    let encoder = unsafe { &mut *encoder };
    encoder.buffer.clear(); // Clear previous data
    let mut record_encoder = RecordEncoder::new(&mut encoder.buffer);

    for i in 0..record_count {
        // records is pointer to first RecordData, this iterated the pointer to i-th RecordData
        let record = unsafe { &*records.add(i) };
        let record_ref = record.to_record_ref();

        // Attempt to encode the record
        if let Err(_) = record_encoder.encode_record(&record_ref) {
            return -2;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn get_encoded_data(
    encoder: *const CRecordEncoder,
    output: *mut u8,
    output_size: *mut usize,
) -> i32 {
    if encoder.is_null() || output_size.is_null() {
        return -1;
    }

    let encoder = unsafe { &*encoder };
    let buffer = &encoder.buffer;

    if !output.is_null() {
        let output_slice = unsafe { std::slice::from_raw_parts_mut(output, buffer.len()) };
        output_slice.copy_from_slice(buffer);
    }

    // Return the size of the buffer
    unsafe {
        *output_size = buffer.len();
    }

    0
}

#[no_mangle]
pub extern "C" fn write_buffer_to_file(
    encoder: *mut CRecordEncoder,
    file_path: *const c_char,
    append: bool,
) -> i32 {
    if encoder.is_null() || file_path.is_null() {
        return -1;
    }

    // Convert the C string file path to a Rust Path
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let path = match c_str.to_str() {
        Ok(s) => Path::new(s),
        Err(_) => return -2,
    };

    let encoder = unsafe { &mut *encoder };
    let record_encoder = RecordEncoder::new(&mut encoder.buffer);

    // Write the buffer to the file
    if let Err(_) = record_encoder.write_to_file(path, append) {
        return -4;
    }

    0
}

/// Destroy the `CRecordEncoder`
#[no_mangle]
pub extern "C" fn destroy_record_encoder(encoder: *mut CRecordEncoder) {
    if encoder.is_null() {
        return;
    }

    // Convert the raw pointer back to a Box and drop it
    unsafe {
        let _ = Box::from_raw(encoder);
    }
}
