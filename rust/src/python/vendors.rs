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

    fn __repr__(&self) -> String {
        format!("<Vendors.{}: '{}'>", self.name(), self.value())
    }

    #[getter]
    fn name(&self) -> String {
        self.as_ref().to_ascii_uppercase()
    }

    #[getter]
    fn value(&self) -> String {
        self.__str__()
    }

    fn to_json(&self) -> String {
        self.as_ref().to_string() // Directly return the string without extra quotes
    }
}
