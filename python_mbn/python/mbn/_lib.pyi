# lib.pyi
from abc import ABC
from typing import Dict, List, Optional
from enum import Enum
from typing import SupportsBytes
import pandas

class Side(Enum):
    ASK: str
    BID: str
    NONE: str

    @classmethod
    def from_str(cls, value: str) -> Side: ...
    @classmethod
    def from_int(cls, value: int) -> Side: ...

class Action(Enum):
    MODIFY: str
    TRADE: str
    FILL: str
    CANCEL: str
    ADD: str
    CLEAR: str

    @classmethod
    def from_str(cls, value: str) -> Action: ...
    @classmethod
    def from_int(cls, value: int) -> Action: ...

class Schema(Enum):
    MBP1: str
    OHLCV1_S: str
    OHLCV1_M: str
    # TBBO: str
    # TRADES: str
    # OHLCV_1H: str
    # OHLCV_1D: str
    # BBO_1S: str
    # BBO_1M: str
    @classmethod
    def from_str(cls, value: str) -> Schema: ...

class RType(Enum):
    MBP1: str
    OHLCV: str

    @classmethod
    def from_int(cls, value: int) -> RType: ...
    @classmethod
    def from_schema(cls, value: Schema) -> RType: ...
    @classmethod
    def from_str(cls, value: str) -> RType: ...

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
    def decode(cls, data: bytes) -> Metadata: ...
    def encode(self) -> bytes: ...
    @property
    def schema(self) -> Schema: ...
    @property
    def start(self) -> int: ...
    @property
    def end(self) -> int: ...
    @property
    def mappings(self) -> SymbolMap: ...

class SymbolMap:
    def __init__(self, map: Dict[int, str]) -> None: ...
    @property
    def map(self) -> Dict: ...
    def get_ticker(self, id: int) -> str: ...

class BufferStore(SupportsBytes):
    def __init__(self, data: bytes) -> None: ...
    def __bytes__(self) -> bytes: ...
    @property
    def metadata(self) -> Metadata: ...
    def decode_to_array(self) -> List[RecordMsg]: ...
    @staticmethod
    def from_file(file_path: str) -> BufferStore: ...
    def decode_to_df(self) -> pandas.DataFrame: ...
    def replay(self) -> Optional[RecordMsg]: ...

class RecordMsg:
    def __init__(self) -> None: ...
    @property
    def hd(self) -> RecordHeader: ...
    @property
    def price(self) -> int: ...

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

class OhlcvMsg(RecordMsg):
    def __init__(
        self,
        hd: RecordHeader,
        open: int,
        high: int,
        low: int,
        close: int,
        volume: int,
    ) -> None: ...
    @property
    def price(self) -> int: ...

class Mbp1Msg(RecordMsg):
    def __init__(
        self,
        hd: RecordHeader,
        price: int,
        size: int,
        action: int,
        side: int,
        depth: int,
        ts_recv: int,
        ts_in_delta: int,
        sequence: int,
        levels: List[BidAskPair],
    ) -> None: ...
    @property
    def price(self) -> int: ...
