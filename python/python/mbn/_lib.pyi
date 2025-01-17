# lib.pyi
from typing import Dict, List, Optional
from enum import Enum
from typing import SupportsBytes
import pandas

class Side(Enum):
    ASK: str
    BID: str
    NONE: str

    @classmethod
    def from_str(cls, value: str) -> "Side": ...
    @classmethod
    def from_int(cls, value: int) -> "Side": ...

class Action(Enum):
    MODIFY: str
    TRADE: str
    FILL: str
    CANCEL: str
    ADD: str
    CLEAR: str

    @classmethod
    def from_str(cls, value: str) -> "Action": ...
    @classmethod
    def from_int(cls, value: int) -> "Action": ...

class Vendors(Enum):
    DATABENTO: str
    YFINANCE: str
    @classmethod
    def from_str(cls, value: str) -> "Vendors": ...

class Dataset(Enum):
    FUTURES: str
    EQUITIES: str
    OPTION: str
    @classmethod
    def from_str(cls, value: str) -> "Dataset": ...

class Schema(Enum):
    MBP1: str
    OHLCV1_S: str
    OHLCV1_M: str
    OHLCV1_H: str
    OHLCV1_D: str
    TRADE: str
    TBBO: str
    BBO1_S: str
    BBO1_M: str
    @classmethod
    def from_str(cls, value: str) -> "Schema": ...

class RType(Enum):
    MBP1: str
    OHLCV: str
    TRADE: str
    TBBO: str
    BBO: str

    @classmethod
    def from_int(cls, value: int) -> "RType": ...
    @classmethod
    def from_schema(cls, value: Schema) -> "RType": ...
    @classmethod
    def from_str(cls, value: str) -> "RType": ...

class SymbolMap:
    def __init__(self, map: Dict[int, str]) -> None: ...

    @property
    def map(self) -> Dict: ...
    def get_ticker(self, id: int) -> str: ...

class Metadata(SupportsBytes):
    def __init__(
        self,
        schema: Schema | None,
        start: int,
        end: int | None = None,
        mappings: SymbolMap | None = None,
    ) -> None: ...
    def __bytes__(self) -> bytes: ...
    @classmethod
    def decode(cls, data: bytes) -> "Metadata": ...
    def encode(self) -> bytes: ...
    @property
    def schema(self) -> Schema: ...
    @property
    def start(self) -> int: ...
    @property
    def end(self) -> int: ...
    @property
    def mappings(self) -> SymbolMap: ...


class RetrieveParams(SupportsBytes):
    def __init__(
        self,
        symbols: List[str],
        start: int,
        end: int,
        schema: Schema,
        dataset: Dataset,
    ) -> None: ...
    @property
    def symbols(self) -> List[str]: ...
    @property
    def start(self) -> int: ...
    @property
    def end(self) -> int: ...
    @property
    def schema(self) -> Schema: ...
    @property
    def dataset(self) -> Dataset: ...


class RecordHeader:
    """docs testing"""
    def __init__(self, instrument_id: int, ts_event: int) -> None: ...
    @property
    def ts_event(self) -> int: ...
    """
    Returns the timestamp of the event.
    """
    @property
    def instrument_id(self) -> int: ...
    """
    Returns the timestamp of the event.
    """
    @property
    def rtype(self) -> RType: ...

class RecordMsg:
    @property
    def hd(self) -> RecordHeader: ...
    @property
    def instrument_id(self) -> int: ...
    @property
    def ts_event(self) -> int: ...
    @property
    def rtype(self) -> RType: ...
    @property
    def pretty_price(self) -> float: ...


class BidAskPair:
    def __init__(
        self,
        bid_px: int,
        ask_px: int,
        bid_sz: int,
        ask_sz: int,
        bid_ct: int,
        ask_ct: int,
    ) -> None: ...
    @property
    def bid_px(self) -> int: ...
    @property
    def pretty_bid_px(self) -> float: ...
    @property
    def ask_px(self) -> int: ...
    @property
    def pretty_ask_px(self) -> float: ...
    @property
    def bid_sz(self) -> int: ...
    @property
    def ask_sz(self) -> int: ...
    @property
    def bid_ct(self) -> int: ...
    @property
    def ask_ct(self) -> int: ...

class OhlcvMsg(RecordMsg):
    def __init__(
        self,
        instrument_id: int,
        ts_event: int,
        open: int,
        high: int,
        low: int,
        close: int,
        volume: int,
    ) -> None: ...
    @property
    def hd(self) -> RecordHeader: ...
    @property
    def instrument_id(self) -> int: ...
    @property
    def ts_event(self) -> int: ...
    @property
    def rtype(self) -> RType: ...
    @property
    def open(self) -> int: ...
    @property
    def pretty_open(self) -> float: ...
    @property
    def high(self) -> int: ...
    @property
    def pretty_high(self) -> float: ...
    @property
    def low(self) -> int: ...
    @property
    def pretty_low(self) -> float: ...
    @property
    def close(self) -> int: ...
    @property
    def pretty_close(self) -> float: ...
    @property
    def volume(self) -> int: ...
    @property
    def pretty_price(self) -> float: ...
    """ 
    Returns the close in dollars.

    Provides polymorphism as other records have a specific price field.

    """

class TradeMsg(RecordMsg):
    def __init__(
        self,
        instrument_id: int,
        ts_event: int,
        price: int,
        size: int,
        action: str,
        side: str,
        depth: int,
        flags: int,
        ts_recv: int,
        ts_in_delta: int,
        sequence: int,
    ) -> None: ...
    @property
    def price(self) -> int: ...
    @property
    def pretty_price(self) -> float: ...
    @property
    def size(self) -> int: ...
    @property
    def action(self) -> int: ...
    @property
    def pretty_action(self) -> Action: ...
    @property
    def pretty_side(self) -> Side: ...
    @property
    def side(self) -> int: ...
    @property
    def depth(self) -> int: ...
    @property
    def flags(self) -> int: ...
    @property
    def ts_recv(self) -> int: ...
    @property
    def ts_in_delta(self) -> int: ...
    @property
    def sequence(self) -> int: ...


class BboMsg(RecordMsg):
    def __init__(
        self,
        instrument_id: int,
        ts_event: int,
        price: int,
        size: int,
        side: str,
        flags: int,
        ts_recv: int,
        sequence: int,
        levels: List[BidAskPair],
    ) -> None: ...
    @property
    def price(self) -> int: ...
    @property
    def pretty_price(self) -> float: ...
    @property
    def size(self) -> int: ...
    @property
    def pretty_side(self) -> Side: ...
    @property
    def side(self) -> int: ...
    @property
    def flags(self) -> int: ...
    @property
    def ts_recv(self) -> int: ...
    @property
    def sequence(self) -> int: ...
    @property
    def levels(self) -> List[BidAskPair]: ...

class Mbp1Msg(RecordMsg):
    def __init__(
        self,
        instrument_id: int,
        ts_event: int,
        price: int,
        size: int,
        action: str,
        side: str,
        depth: int,
        flags: int,
        ts_recv: int,
        ts_in_delta: int,
        sequence: int,
        discrimincator: int,
        levels: List[BidAskPair],
    ) -> None: ...
    @property
    def price(self) -> int: ...
    @property
    def pretty_price(self) -> float: ...
    @property
    def size(self) -> int: ...
    @property
    def action(self) -> int: ...
    @property
    def pretty_action(self) -> Action: ...
    @property
    def pretty_side(self) -> Side: ...
    @property
    def side(self) -> int: ...
    @property
    def depth(self) -> int: ...
    @property
    def flags(self) -> int: ...
    @property
    def ts_recv(self) -> int: ...
    @property
    def ts_in_delta(self) -> int: ...
    @property
    def sequence(self) -> int: ...
    @property
    def discriminator(self) -> int: ...
    @property
    def levels(self) -> List[BidAskPair]: ...

class BufferStore(SupportsBytes):
    def __init__(self, data: bytes) -> None: ...
    def __bytes__(self) -> bytes: ...
    @property
    def metadata(self) -> Metadata: ...
    def decode_to_array(self) -> List[RecordMsg]: ...
    def write_to_file(self, file_path: str) -> None: ...
    @staticmethod
    def from_file(file_path: str) -> "BufferStore": ...
    def decode_to_df(self, pretty_ts: bool, pretty_px: bool) -> pandas.DataFrame: ...
    def replay(self) -> Optional[RecordMsg]: ...

# -- Trading -- 

class SignalInstructions:
    def __init__(
        self,
        ticker: str,
        order_type: str,
        action: str,
        trade_id: int,
        leg_id: int,
        weight: int,
        quantity: int,
        limit_price: str,
        aux_price: str,
    ) -> None: ...

class Signals:
    def __init__(
         self,
        timestamp: int, 
        trade_instructions: List[SignalInstructions]
    ) -> None: ...

class Trades:
    def __init__(
        self,
        trade_id: int,
        leg_id: int,
        timestamp: int,
        ticker: str,
        quantity: int,
        avg_price: int,
        trade_value: int,
        action: str,
        fees: int,
    ) -> None: ...

class TimeseriesStats: 
    def __init__(
        self,
        timestamp: int,
        equity_value: int,
        percent_drawdown: int,
        cumulative_return: int,
        period_return: int,
    ) -> None: ...

class StaticStats:
    def __init__(
        self,
        total_trades: int,
        total_winning_trades: int,
        total_losing_trades: int,
        avg_profit: int,
        avg_profit_percent: int,
        avg_gain: int,
        avg_gain_percent: int,
        avg_loss: int,
        avg_loss_percent: int,
        profitability_ratio: int,
        profit_factor: int,
        profit_and_loss_ratio: int,
        total_fees: int,
        net_profit: int,
        beginning_equity: int,
        ending_equity: int,
        total_return: int,
        daily_standard_deviation_percentage: int,
        annual_standard_deviation_percentage: int,
        max_drawdown_percentage_period: int,
        max_drawdown_percentage_daily: int,
        sharpe_ratio: int,
        sortino_ratio: int,
    ) -> None: ...

class Parameters:
    def __init__(self,        
        strategy_name: str,
        capital: int,
        schema: str,
        data_type: str,
        start: int,
        end: int,
        tickers: List[str],
    ) -> None: ...

class BacktestMetaData:
    def __init__(
        self,
        backtest_id: int,
        backtest_name: str,
        parameters: Parameters,
        static_stats: StaticStats,
    ) -> None: ...

class BacktestData:
    def __init__(
        self,
        metadata: BacktestMetaData,
        period_timeseries_stats: List[TimeseriesStats],
        daily_timeseries_stats: List[TimeseriesStats],
        trades: List[Trades],
        signals: List[Signals]
    ) -> None: ...


class PyBacktestEncoder:
    def __init__() -> None: ...
    def encode_backtest(self, backtest: BacktestData) -> List[int]: ...

class AccountSummary:
    def __init__(self,        
        currency: str,
        start_timestamp: int,
        start_buying_power: int,
        start_excess_liquidity: int,
        start_full_available_funds: int,
        start_full_init_margin_req: int,
        start_full_maint_margin_req: int,
        start_futures_pnl: int,
        start_net_liquidation: int,
        start_total_cash_balance: int,
        start_unrealized_pnl: int,
        end_timestamp: int,
        end_buying_power: int,
        end_excess_liquidity: int,
        end_full_available_funds: int,
        end_full_init_margin_req: int,
        end_full_maint_margin_req: int,
        end_futures_pnl: int,
        end_net_liquidation: int,
        end_total_cash_balance: int,
        end_unrealized_pnl: int,
    ) -> None: ...

class LiveData:
    def __init__(
        self,
        live_id: Optional[int],
        parameters: Parameters,
        trades: List[Trades],
        signals: List[Signals],
        account: AccountSummary,
    ) -> None: ...


