use crate::enums::Schema;
use crate::symbols::SymbolMap;

#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(feature = "python", pyclass(get_all, set_all, dict, module = "mbn"))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Metadata {
    pub schema: Schema,
    pub start: u64,
    pub end: u64,
    pub mappings: SymbolMap,
}

impl Metadata {
    pub fn new(schema: Schema, start: u64, end: u64, mappings: SymbolMap) -> Self {
        Metadata {
            schema,
            start,
            end,
            mappings,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.schema as u8);
        bytes.extend_from_slice(&self.start.to_le_bytes());
        bytes.extend_from_slice(&self.end.to_le_bytes());
        bytes.extend_from_slice(&self.mappings.serialize());
        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        let mut offset = 0;
        let schema = Schema::try_from(bytes[offset]).unwrap();
        offset += 1;
        let start = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let end = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let mappings = SymbolMap::deserialize(bytes, &mut offset);
        Metadata {
            schema,
            start,
            end,
            mappings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_encoding() {
        let mut symbol_map = SymbolMap::new();
        symbol_map.add_instrument("AAPL", 1);
        symbol_map.add_instrument("TSLA", 2);

        let metadata = Metadata::new(Schema::Ohlcv1S, 1234567898765, 123456765432, symbol_map);

        // Test
        let bytes = metadata.serialize();
        let decoded = Metadata::deserialize(&bytes);

        // Validate
        assert_eq!(metadata, decoded);
    }
}
