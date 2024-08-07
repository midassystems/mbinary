use crate::decode::CombinedDecoder;
use crate::metadata::Metadata;
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::io::Cursor;

//TODO :
//1. Decode to a dataframe structure

#[cfg_attr(feature = "python", pyo3::pyclass(module = "mbn"))]
pub struct BufferStore {
    buffer: Vec<u8>,
    metadata: Metadata,
    decoder: CombinedDecoder<Cursor<Vec<u8>>>,
}

#[pymethods]
impl BufferStore {
    #[new]
    pub fn py_new(data: &Bound<PyBytes>) -> Self {
        let buffer = data.as_bytes().to_vec();
        let cursor = Cursor::new(buffer.clone());
        let mut decoder = CombinedDecoder::new(cursor);
        let metadata = decoder
            .decode_metadata()
            .expect("Error decoding metadata")
            .unwrap();

        BufferStore {
            buffer,
            metadata,
            decoder,
        }
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.metadata.clone().into_py(py))
    }

    pub fn decode_to_array(&mut self) -> PyResult<Vec<PyObject>> {
        let decoded = self
            .decoder
            .decode_all_records()
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Python::with_gil(|py| {
            Ok(decoded
                .into_iter()
                .map(|record| record.into_py(py))
                .collect())
        })
    }

    pub fn write_to_file(&self, file_path: &str) -> PyResult<()> {
        std::fs::write(file_path, &self.buffer).map_err(|e| PyIOError::new_err(e.to_string()))
    }

    #[staticmethod]
    pub fn from_file(file_path: &str, py: Python) -> PyResult<Self> {
        // let buffer = std::fs::read(file_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let buffer = std::fs::read(file_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let py_bytes = PyBytes::new_bound(py, &buffer);
        Ok(BufferStore::py_new(&py_bytes))
    }

    // pub fn decode_to_dataframe(&mut self, py: Python) -> PyResult<DataFrame> {
    //     // Decode all records
    //     let records: Vec<RecordEnum> = self
    //         .decoder
    //         .decode_all_records()
    //         .map_err(|e| PyIOError::new_err(e.to_string()))?;

    //     // Convert RecordEnum to their underlying message types and collect them
    //     let ohlcv_records: Vec<OhlcvMsg> = records
    //         .into_iter()
    //         .filter_map(|rec| {
    //             if let RecordEnum::Ohlcv(msg) = rec {
    //                 Some(msg)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();

    //     // Create vectors to hold each column
    //     let lengths: Vec<u32> = ohlcv_records
    //         .iter()
    //         .map(|msg| msg.hd.length as u32)
    //         .collect();
    //     let rtypes: Vec<u32> = ohlcv_records
    //         .iter()
    //         .map(|msg| msg.hd.rtype as u32)
    //         .collect();
    //     let instrument_ids: Vec<u32> = ohlcv_records
    //         .iter()
    //         .map(|msg| msg.hd.instrument_id as u32)
    //         .collect();
    //     let ts_events: Vec<u64> = ohlcv_records.iter().map(|msg| msg.hd.ts_event).collect();
    //     let opens: Vec<i64> = ohlcv_records.iter().map(|msg| msg.open).collect();
    //     let highs: Vec<i64> = ohlcv_records.iter().map(|msg| msg.high).collect();
    //     let lows: Vec<i64> = ohlcv_records.iter().map(|msg| msg.low).collect();
    //     let closes: Vec<i64> = ohlcv_records.iter().map(|msg| msg.close).collect();
    //     let volumes: Vec<u64> = ohlcv_records.iter().map(|msg| msg.volume).collect();

    //     // Create a DataFrame
    //     let df = DataFrame::new(vec![
    //         Series::new("length", lengths),
    //         Series::new("rtype", rtypes),
    //         Series::new("instrument_id", instrument_ids),
    //         Series::new("ts_event", ts_events),
    //         Series::new("open", opens),
    //         Series::new("high", highs),
    //         Series::new("low", lows),
    //         Series::new("close", closes),
    //         Series::new("volume", volumes),
    //     ])
    //     .map_err(|e| PyIOError::new_err(e.to_string()))?;
    //     println!("{:?}", df);

    //     // Convert the DataFrame to a PyDataFrame (or similar PyO3 compatible structure)
    //     // let py_df = PyDataFrame::new(df);

    //     Ok(df)
    // }
}

// #[pyclass]
// struct PyDataFrame {
//     df: DataFrame,
// }

// #[pymethods]
// impl PyDataFrame {
//     #[new]
//     fn new(df: DataFrame) -> Self {
//         PyDataFrame { df }
//     }

//     pub fn to_string(&self) -> String {
//         format!("{:?}", self.df)
//     }
// }
