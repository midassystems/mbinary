use crate::decode::CombinedDecoder;
use crate::metadata::Metadata;
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::io::Cursor;

#[cfg_attr(feature = "python", pyo3::pyclass(module = "mbn"))]
pub struct BufferStore {
    buffer: Vec<u8>,
    metadata: Metadata,
    decoder: CombinedDecoder<Cursor<Vec<u8>>>,
}

#[pymethods]
impl BufferStore {
    #[new]
    pub fn py_new(data: &Bound<PyBytes>) -> Self {
        let buffer = data.as_bytes().to_vec();
        let cursor = Cursor::new(buffer.clone());
        let mut decoder = CombinedDecoder::new(cursor);
        let metadata = decoder
            .decode_metadata()
            .expect("Error decoding metadata")
            .unwrap();

        BufferStore {
            buffer,
            metadata,
            decoder,
        }
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.metadata.clone().into_py(py))
    }

    pub fn decode_to_array(&mut self) -> PyResult<Vec<PyObject>> {
        let decoded = self
            .decoder
            .decode_all_records()
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Python::with_gil(|py| {
            Ok(decoded
                .into_iter()
                .map(|record| record.into_py(py))
                .collect())
        })
    }

    pub fn replay(&mut self, py: Python) -> Option<PyObject> {
        let mut iter = self.decoder.decode_iterator();

        match iter.next() {
            Some(Ok(record)) => Some(record.into_py(py)),
            Some(Err(e)) => {
                PyIOError::new_err(e.to_string()).restore(py);
                None
            }
            None => None, // End of iteration
        }
    }

    pub fn decode_to_df(&mut self, py: Python) -> PyResult<PyObject> {
        // Use the existing `decode_to_array` to get the list of PyObject
        let flat_array: Vec<PyObject> = self.decode_to_array()?;

        // Map instrument_id to symbols using the metadata mappings
        let mappings = self.metadata.mappings.map.clone();

        // Convert to DataFrame using the dictionaries returned by `__dict__`
        let dicts: Vec<_> = flat_array
            .iter()
            .map(|obj| {
                let dict_obj = obj.call_method0(py, "__dict__")?; // Create a binding for the temporary value
                let dict = dict_obj.downcast_bound::<PyDict>(py)?; // Now use the bound value

                // Get the instrument_id from the dict, handling the PyResult<Option<PyAny>>
                if let Some(instrument_id_obj) = dict.get_item("instrument_id")? {
                    // Extract the instrument_id as a u32
                    let instrument_id: u32 = instrument_id_obj.extract()?;

                    // Set the corresponding symbol
                    if let Some(symbol) = mappings.get(&instrument_id) {
                        dict.set_item("symbol", symbol)?;
                    }
                }
                Ok(dict.to_object(py))
            })
            .collect::<PyResult<Vec<_>>>()?;

        let pandas = py.import_bound("pandas")?;
        let df = pandas.call_method1("DataFrame", (dicts,))?;
        Ok(df.into())
    }

    pub fn write_to_file(&self, file_path: &str) -> PyResult<()> {
        std::fs::write(file_path, &self.buffer).map_err(|e| PyIOError::new_err(e.to_string()))
    }

    #[staticmethod]
    pub fn from_file(file_path: &str, py: Python) -> PyResult<Self> {
        // let buffer = std::fs::read(file_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let buffer = std::fs::read(file_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let py_bytes = PyBytes::new_bound(py, &buffer);
        Ok(BufferStore::py_new(&py_bytes))
    }
}
