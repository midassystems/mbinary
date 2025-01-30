use crate::decode::MetadataDecoder;
use crate::encode::MetadataEncoder;
use crate::enums::{Dataset, Schema};
use crate::metadata::Metadata;
use crate::symbols::SymbolMap;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

#[pymethods]
impl Metadata {
    #[new]
    fn py_new(schema: Schema, dataset: Dataset, start: u64, end: u64, mappings: SymbolMap) -> Self {
        Metadata {
            schema,
            dataset,
            start,
            end,
            mappings,
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }

    fn __bytes__(&self, py: Python<'_>) -> PyResult<Py<PyBytes>> {
        self.py_encode(py)
    }

    #[pyo3(name = "encode")]
    fn py_encode(&self, py: Python<'_>) -> PyResult<Py<PyBytes>> {
        let mut buffer = Vec::new();
        let mut encoder = MetadataEncoder::new(&mut buffer);
        encoder.encode_metadata(self)?;
        Ok(PyBytes::new(py, buffer.as_slice()).into())
    }

    #[pyo3(name = "decode")]
    #[classmethod]
    fn py_decode(_cls: &Bound<PyType>, data: &Bound<PyBytes>) -> PyResult<Metadata> {
        let reader = std::io::BufReader::new(data.as_bytes());
        let mut decoder = MetadataDecoder::new(reader);
        let metadata = decoder.decode()?.unwrap();
        Ok(metadata)
    }
}
