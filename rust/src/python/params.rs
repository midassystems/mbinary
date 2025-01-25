use crate::enums::{Dataset, Schema, Stype};
use crate::params::RetrieveParams;
use pyo3::prelude::*;

#[pymethods]
impl RetrieveParams {
    #[new]
    fn py_new(
        symbols: Vec<String>,
        start: &str,
        end: &str,
        schema: Schema,
        dataset: Dataset,
        stype: Stype,
    ) -> PyResult<Self> {
        Ok(
            RetrieveParams::new(symbols, start, end, schema, dataset, stype).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!(
                    "Failed to create RetrieveParams : {}",
                    e
                ))
            })?,
        )
    }

    /// Convert `RetrieveParams` to JSON string
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize to JSON: {}", e))
        })
    }

    /// Create `RetrieveParams` from JSON string
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "Failed to deserialize from JSON: {}",
                e
            ))
        })
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}
