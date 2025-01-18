use crate::vendors::Vendors;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;

#[pymethods]
impl Vendors {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: &Bound<PyAny>) -> PyResult<Self> {
        let vendor_str: String = value.extract()?;
        Vendors::from_str(&vendor_str).map_err(|e| PyValueError::new_err(e.extract_message()))
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}
