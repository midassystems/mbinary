use crate::enums::{Dataset, Schema};
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
    ) -> PyResult<Self> {
        Ok(
            RetrieveParams::new(symbols, start, end, schema, dataset).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!(
                    "Failed to create RetrieveParams : {}",
                    e
                ))
            })?,
        )
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}
