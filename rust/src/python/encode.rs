use crate::encode::RecordEncoder;
use crate::record_ref::RecordRef;
use crate::records::Mbp1Msg; // Your existing RecordEncoder implementation
use pyo3::prelude::*; // PyO3 essentials

/// Python-facing wrapper for RecordEncoder
#[cfg_attr(feature = "python", pyclass(module = "mbn"))]
pub struct PyRecordEncoder {
    buffer: Vec<u8>, // Owned buffer
}

#[pymethods]
impl PyRecordEncoder {
    /// Constructor for PyRecordEncoder
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyRecordEncoder {
            buffer: Vec::new(), // Initialize with an empty buffer
        })
    }

    /// Encodes multiple records
    fn encode_records(&mut self, records: Vec<Mbp1Msg>) -> PyResult<()> {
        self.buffer.clear(); // Clear the buffer for new encoding

        let mut encoder = RecordEncoder::new(&mut self.buffer); // Create a temporary encoder

        for record in records {
            let record_ref = RecordRef::from(&record);
            encoder.encode_record(&record_ref).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to encode record: {}", e))
            })?;
        }
        Ok(())
    }

    /// Retrieve the encoded data as bytes
    fn get_encoded_data(&self) -> PyResult<Vec<u8>> {
        Ok(self.buffer.clone()) // Return a copy of the buffer
    }
}
