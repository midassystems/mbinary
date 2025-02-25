use crate::decode::Decoder;
use crate::metadata::Metadata;
use crate::utils::unix_nanos_to_date;
use crate::PRICE_SCALE;
use pyo3::exceptions::PyIOError;
use pyo3::types::{PyBytes, PyDict};
use pyo3::{prelude::*, IntoPyObjectExt};
use std::io::Cursor;

#[cfg_attr(feature = "python", pyo3::pyclass(module = "mbinary"))]
pub struct BufferStore {
    buffer: Vec<u8>,
    metadata: Metadata,
    decoder: Decoder<Cursor<Vec<u8>>>,
}

#[pymethods]
impl BufferStore {
    #[new]
    pub fn py_new(data: &Bound<PyBytes>) -> PyResult<Self> {
        let buffer = data.as_bytes().to_vec();
        let cursor = Cursor::new(buffer.clone());
        let mut decoder = Decoder::new(cursor)?;
        let metadata = decoder.metadata().unwrap();

        Ok(BufferStore {
            buffer,
            metadata,
            decoder,
        })
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.metadata.clone().into_py_any(py)?)
    }

    pub fn decode_to_array(&mut self) -> PyResult<Vec<PyObject>> {
        let decoded = self
            .decoder
            .decode()
            .map_err(|e| PyIOError::new_err(e.to_string()))?;

        Python::with_gil(|py| {
            decoded
                .into_iter()
                .map(|record| record.into_py_any(py)) // No `?` inside map
                .collect::<PyResult<Vec<_>>>() // Collect into PyResult<Vec<PyObject>>
        })
    }

    pub fn replay(&mut self, py: Python) -> Option<PyObject> {
        let mut iter = self.decoder.decode_iterator();

        match iter.next() {
            Some(Ok(record)) => match record.into_py_any(py) {
                Ok(obj) => Some(obj), // Extract `PyObject` from `Ok`
                Err(e) => {
                    e.restore(py); // Restore the error
                    None
                }
            },
            Some(Err(e)) => {
                PyIOError::new_err(e.to_string()).restore(py);
                None
            }
            None => None, // End of iteration
        }
    }

    pub fn decode_to_df(
        &mut self,
        py: Python,
        pretty_ts: bool,
        pretty_px: bool,
    ) -> PyResult<PyObject> {
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
                // Outputs char instead of number
                if let Some(action_obj) = dict.get_item("action")? {
                    let action: u8 = action_obj.extract()?;
                    dict.set_item("action", action as char)?;
                }

                if let Some(side_obj) = dict.get_item("side")? {
                    let side: u8 = side_obj.extract()?;
                    dict.set_item("side", side as char)?;
                }

                if pretty_ts {
                    if let Some(ts_obj) = dict.get_item("ts_event")? {
                        let ts_event: i64 = ts_obj.extract()?;
                        let iso: String = unix_nanos_to_date(ts_event).map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))
                        })?;
                        dict.set_item("ts_event", iso)?;
                    }
                }
                // Convert price fields if pretty = True
                if pretty_px {
                    if let Some(open_obj) = dict.get_item("open")? {
                        let open: i64 = open_obj.extract()?;
                        dict.set_item("open", (open as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(high_obj) = dict.get_item("high")? {
                        let high: i64 = high_obj.extract()?;
                        dict.set_item("high", (high as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(low_obj) = dict.get_item("low")? {
                        let low: i64 = low_obj.extract()?;
                        dict.set_item("low", (low as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(close_obj) = dict.get_item("close")? {
                        let close: i64 = close_obj.extract()?;
                        dict.set_item("close", (close as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(price_obj) = dict.get_item("price")? {
                        let price: i64 = price_obj.extract()?;
                        dict.set_item("price", (price as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(ask_px_obj) = dict.get_item("ask_px")? {
                        let ask_px: i64 = ask_px_obj.extract()?;
                        dict.set_item("ask_px", (ask_px as f64) / (PRICE_SCALE as f64))?;
                    }
                    if let Some(bid_px_obj) = dict.get_item("bid_px")? {
                        let bid_px: i64 = bid_px_obj.extract()?;
                        dict.set_item("bid_px", (bid_px as f64) / (PRICE_SCALE as f64))?;
                    }
                }

                Ok(dict.clone().into_pyobject(py)?)
            })
            .collect::<PyResult<Vec<_>>>()?;

        let pandas = py.import("pandas")?;
        let df = pandas.call_method1("DataFrame", (dicts,))?;
        Ok(df.into())
    }

    pub fn write_to_file(&self, file_path: &str) -> PyResult<()> {
        std::fs::write(file_path, &self.buffer).map_err(|e| PyIOError::new_err(e.to_string()))
    }

    #[staticmethod]
    pub fn from_file(file_path: &str, py: Python) -> PyResult<Self> {
        let buffer = std::fs::read(file_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let py_bytes = PyBytes::new(py, &buffer);
        Ok(BufferStore::py_new(&py_bytes)?)
    }
}
