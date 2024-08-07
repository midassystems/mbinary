use crate::symbols::SymbolMap;
use pyo3::prelude::*;
// use pyo3::prelude::{pymethods, PyResult};
use std::collections::HashMap;

#[pymethods]
impl SymbolMap {
    #[new]
    fn py_new(map: HashMap<u32, String>) -> Self {
        SymbolMap { map }
    }

    fn __str__(&self) -> String {
        let mut map_str = String::from("{");
        for (k, v) in &self.map {
            map_str.push_str(&format!("{}: \"{}\", ", k, v));
        }
        if map_str.len() > 1 {
            map_str.truncate(map_str.len() - 2); // Remove the trailing comma and space
        }
        map_str.push('}');
        map_str
    }

    fn __eq__(&self, value: &Bound<PyAny>) -> PyResult<bool> {
        if let Ok(other) = value.extract::<SymbolMap>() {
            Ok(self.map == other.map)
        } else {
            Ok(false)
        }
    }

    fn get_ticker(&self, id: u8) -> PyResult<String> {
        let id = SymbolMap::get_instrument_ticker(&self, id as u32).unwrap();
        Ok(id)
    }
}
