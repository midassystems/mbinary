use crate::enums::{Action, Dataset, RType, Schema, Side, Stype};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;

#[pymethods]
impl Side {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: char) -> PyResult<Self> {
        Side::try_from(value as u8)
            .map_err(|_| PyValueError::new_err(format!("Unknown Side value: '{}'", value)))
    }

    #[classmethod]
    fn from_int(_cls: &Bound<'_, PyType>, value: u8) -> PyResult<Self> {
        let char: char = value as char;
        Self::py_from_str(_cls, char)
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<Side.{}: '{}'>", self.name(), self.value())
    }

    #[getter]
    fn name(&self) -> String {
        self.as_ref().to_ascii_uppercase()
    }

    #[getter]
    fn value(&self) -> String {
        self.__str__()
    }
}

#[pymethods]
impl Action {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: char) -> PyResult<Self> {
        Action::try_from(value as u8)
            .map_err(|_| PyValueError::new_err(format!("Unknown Action value: '{}'", value)))
    }

    #[classmethod]
    pub fn from_int(_cls: &Bound<'_, PyType>, value: u8) -> PyResult<Self> {
        let char: char = value as char;
        Self::py_from_str(_cls, char)
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<Action.{}: '{}'>", self.name(), self.value())
    }

    #[getter]
    fn name(&self) -> String {
        self.as_ref().to_ascii_uppercase()
    }

    #[getter]
    fn value(&self) -> String {
        self.__str__()
    }
}

#[pymethods]
impl Schema {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: &Bound<PyAny>) -> PyResult<Self> {
        let schema_str: String = value.extract()?;
        Schema::from_str(&schema_str).map_err(|e| PyValueError::new_err(e.extract_message()))
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<Schema.{}: '{}'>", self.name(), self.value())
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

#[pymethods]
impl Stype {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: &Bound<PyAny>) -> PyResult<Self> {
        let stype_str: String = value.extract()?;
        Stype::from_str(&stype_str).map_err(|e| PyValueError::new_err(e.extract_message()))
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<Stype.{}: '{}'>", self.name(), self.value())
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

#[pymethods]
impl Dataset {
    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: &Bound<PyAny>) -> PyResult<Self> {
        let dataset_str: String = value.extract()?;
        Dataset::from_str(&dataset_str).map_err(|e| PyValueError::new_err(e.extract_message()))
    }
    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<Dataset.{}: '{}'>", self.name(), self.value())
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

#[pymethods]
impl RType {
    #[classmethod]
    fn from_int(_cls: &Bound<'_, PyType>, value: u8) -> PyResult<Self> {
        RType::try_from(value)
            .map_err(|_| PyValueError::new_err(format!("Unknown RType value: {}", value)))
    }

    #[classmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(_cls: &Bound<'_, PyType>, value: &str) -> PyResult<Self> {
        RType::from_str(value).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    fn from_schema(_cls: &Bound<'_, PyType>, value: &Bound<PyAny>) -> PyResult<Self> {
        let schema: Schema = value.extract()?;
        Ok(RType::from(schema))
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __repr__(&self) -> String {
        format!("<RType.{}: '{}'>", self.name(), self.value())
    }

    #[getter]
    fn name(&self) -> String {
        self.as_ref().to_ascii_uppercase()
    }

    #[getter]
    fn value(&self) -> String {
        self.__str__()
    }
}
