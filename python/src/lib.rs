use mbinary::{
    backtest::{
        BacktestData, BacktestMetaData, Parameters, SignalInstructions, Signals, StaticStats,
        TimeseriesStats, Trades,
    },
    enums::{Action, Dataset, RType, Schema, Side, Stype},
    live::{AccountSummary, LiveData},
    metadata::Metadata,
    params::RetrieveParams,
    python::backest_encode::PyBacktestEncoder,
    python::buffer::BufferStore,
    python::encode::{PyMetadataEncoder, PyRecordEncoder},
    python::records::RecordMsg,
    records::{BboMsg, BidAskPair, Mbp1Msg, OhlcvMsg, RecordHeader, TbboMsg, TradeMsg},
    symbols::SymbolMap,
    vendors::Vendors,
    PRICE_SCALE, QUANTITY_SCALE,
};
use pyo3::{prelude::*, PyClass};

// ensure a module was specified, otherwise it defaults to builtins
fn checked_add_class<T: PyClass>(m: &Bound<PyModule>) -> PyResult<()> {
    assert_eq!(T::MODULE.unwrap(), "mbinary");
    m.add_class::<T>()
}

#[pymodule] // The name of the function must match `lib.name` in `Cargo.toml`
#[pyo3(name = "_lib")]
fn python_mbinary(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    checked_add_class::<Side>(m)?;
    checked_add_class::<Vendors>(m)?;
    checked_add_class::<Stype>(m)?;
    checked_add_class::<Dataset>(m)?;
    checked_add_class::<Action>(m)?;
    checked_add_class::<Schema>(m)?;
    checked_add_class::<RType>(m)?;
    checked_add_class::<SymbolMap>(m)?;
    checked_add_class::<Metadata>(m)?;
    checked_add_class::<RecordHeader>(m)?;
    checked_add_class::<OhlcvMsg>(m)?;
    checked_add_class::<Mbp1Msg>(m)?;
    checked_add_class::<TradeMsg>(m)?;
    checked_add_class::<TbboMsg>(m)?;
    checked_add_class::<BboMsg>(m)?;
    checked_add_class::<BidAskPair>(m)?;
    checked_add_class::<RetrieveParams>(m)?;
    checked_add_class::<BufferStore>(m)?;
    checked_add_class::<RecordMsg>(m)?;
    checked_add_class::<BacktestData>(m)?;
    checked_add_class::<BacktestMetaData>(m)?;
    checked_add_class::<StaticStats>(m)?;
    checked_add_class::<Parameters>(m)?;
    checked_add_class::<TimeseriesStats>(m)?;
    checked_add_class::<Trades>(m)?;
    checked_add_class::<Signals>(m)?;
    checked_add_class::<SignalInstructions>(m)?;
    checked_add_class::<LiveData>(m)?;
    checked_add_class::<AccountSummary>(m)?;
    checked_add_class::<PyRecordEncoder>(m)?;
    checked_add_class::<PyMetadataEncoder>(m)?;
    checked_add_class::<PyBacktestEncoder>(m)?;
    let _ = m.add("PRICE_SCALE", PRICE_SCALE);
    let _ = m.add("QUANTITY_SCALE", QUANTITY_SCALE);

    Ok(())
}
