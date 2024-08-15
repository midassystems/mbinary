use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[cfg(feature = "python")]
use pyo3::pyclass;

/// Struct representing a financial instrument.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Instrument {
    /// Instrument ticker.
    pub ticker: String,
    /// Instrument name e.g. Apple Inc.
    pub name: String,
    /// Midas unique instrument id number.
    pub instrument_id: Option<u32>,
}

impl Instrument {
    pub fn new(ticker: &str, name: &str, instrument_id: Option<u32>) -> Self {
        Self {
            ticker: ticker.to_string(),
            name: name.to_string(),
            instrument_id,
        }
    }
}

/// Struct created by Midas server to map instrument ids to tickers.
#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolMap {
    /// Maps {id : ticker}.
    pub map: HashMap<u32, String>,
}

impl SymbolMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_instrument(&mut self, ticker: &str, id: u32) {
        self.map.insert(id, ticker.to_string());
    }

    pub fn get_instrument_ticker(&self, id: u32) -> Option<String> {
        self.map.get(&id).cloned()
    }

    /// Binary encodes struct for response, shouldn't be used directly.
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let map_len = self.map.len() as u32;
        bytes.extend_from_slice(&map_len.to_le_bytes());
        for (key, value) in &self.map {
            bytes.extend_from_slice(&key.to_le_bytes());
            let value_len = value.len() as u32;
            bytes.extend_from_slice(&value_len.to_le_bytes());
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes
    }

    /// Decodes the binary response from Midas server, shouldn't need to be used directly.
    pub fn deserialize(bytes: &[u8], offset: &mut usize) -> Self {
        let map_len = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap()) as usize;
        *offset += 4;
        let mut map = HashMap::with_capacity(map_len);
        for _ in 0..map_len {
            let key = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
            *offset += 4;
            let value_len =
                u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap()) as usize;
            *offset += 4;
            let value = String::from_utf8(bytes[*offset..*offset + value_len].to_vec()).unwrap();
            *offset += value_len;
            map.insert(key, value);
        }
        SymbolMap { map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument() {
        // Test
        let ticker = "AAPL";
        let name = "Apple Inc.";
        let instrument = Instrument::new(ticker, name, None);

        // Validate
        assert_eq!(instrument.ticker, ticker);
        assert_eq!(instrument.name, name);
        assert_eq!(instrument.instrument_id, None);
    }

    #[test]
    fn test_symbol_map() {
        let appl = "AAPL";
        let tsla = "TSLA";

        // Test
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument(appl, 1);
        symbol_map.add_instrument(tsla, 2);

        // Validate
        let ticker1 = symbol_map.get_instrument_ticker(1).unwrap();
        assert_eq!(&ticker1, appl);

        let ticker2 = symbol_map.get_instrument_ticker(2).unwrap();
        assert_eq!(&ticker2, tsla);
    }
}
