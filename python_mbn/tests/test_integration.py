import unittest
import pandas as pd
from mbn import Side, Action, Schema, RType, SymbolMap, Metadata, BufferStore
from mbn import OhlcvMsg
from typing import List

from python.mbn._lib import Mbp1Msg


class IntegrationTests(unittest.TestCase):
    def setUp(self) -> None:
        return super().setUp()

    def test_side(self):
        # Direct instantiation
        bid = Side.BID
        self.assertEqual(bid, Side.BID)

        # from str
        ask = Side.from_str("A")
        self.assertEqual(ask, Side.ASK)

        # from int
        ask = Side.from_int(65)
        self.assertEqual(ask, Side.ASK)

        # Error
        with self.assertRaises(ValueError):
            Side.from_str("T")

        # with self.assertRaises(TypeError):
        #     Side.from_str(9)

    def test_action(self):
        # Direct instantiation
        modify = Action.MODIFY
        self.assertEqual(modify, Action.MODIFY)

        # from str
        add = Action.from_str("A")
        self.assertEqual(add, Action.ADD)

        # from int
        add = Action.from_int(65)
        self.assertEqual(add, Action.ADD)

        # Error
        with self.assertRaises(ValueError):
            Action.from_str("dj")

    def test_schema(self):
        # instantiation
        mbp_1 = Schema.MBP1
        self.assertEqual(mbp_1, Schema.MBP1)

        # from str
        ohlcv = Schema.from_str("ohlcv-1s")
        self.assertEqual(ohlcv, Schema.OHLCV1_S)

        # __str__
        schema = Schema.OHLCV1_S.__str__()
        self.assertEqual(schema, "ohlcv-1s")

        # Error
        with self.assertRaises(ValueError):
            Schema.from_str("ohlcv-12345s")

    def test_rtype(self):
        # from int
        rtype = RType.from_int(0x01)
        self.assertEqual(rtype, RType.MBP1)

        # from str
        rtype = RType.from_str("ohlcv")
        self.assertEqual(rtype, RType.OHLCV)

        # from schema
        rtype = RType.from_schema(Schema.from_str("ohlcv-1s"))
        self.assertEqual(rtype, RType.OHLCV)

        # Errors
        with self.assertRaises(ValueError):
            RType.from_int(0x09)

        with self.assertRaises(ValueError):
            RType.from_str("olghd")

    def test_metadata(self):
        symbol_map = SymbolMap({1: "AAPL", 2: "TSLA"})

        # Test
        metadata = Metadata(
            Schema.from_str("ohlcv-1s"),
            1234567654321,
            987654345676543456,
            symbol_map,
        )
        encoded = metadata.encode()
        decoded_metadata = metadata.decode(encoded)

        # Validate
        self.assertEqual(decoded_metadata.start, metadata.start)
        self.assertEqual(decoded_metadata.schema, metadata.schema)
        self.assertEqual(decoded_metadata.mappings, metadata.mappings)
        self.assertEqual(decoded_metadata.end, metadata.end)

    def test_symbol_map(self):
        # Test
        symbol_map = SymbolMap({1: "AAPL", 2: "TSLA"})

        # Validate
        ticker_1 = symbol_map.get_ticker(1)
        self.assertEqual(ticker_1, "AAPL")

        ticker_2 = symbol_map.get_ticker(2)
        self.assertEqual(ticker_2, "TSLA")

        mappings = symbol_map.map
        self.assertEqual(mappings, mappings)

    def test_buffer_store_file(self):
        # Binary
        bin = [
            2,
            141,
            38,
            251,
            113,
            31,
            1,
            0,
            0,
            248,
            189,
            152,
            190,
            28,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            65,
            65,
            80,
            76,
            2,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            84,
            83,
            76,
            65,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            14,
            2,
            0,
            0,
            1,
            0,
            0,
            0,
            212,
            241,
            180,
            96,
            0,
            0,
            0,
            0,
            100,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            200,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            50,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            150,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            232,
            3,
            0,
            0,
            0,
            0,
            0,
            0,
            14,
            2,
            0,
            0,
            2,
            0,
            0,
            0,
            213,
            241,
            180,
            96,
            0,
            0,
            0,
            0,
            110,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            210,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            55,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            155,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            76,
            4,
            0,
            0,
            0,
            0,
            0,
            0,
        ]

        # Write bin file
        buffer_obj = BufferStore(bytes(bin))
        buffer_obj.write_to_file("test.bin")

        # Test
        new_buffer = BufferStore.from_file("test.bin")
        metadata = new_buffer.metadata
        ohlcv_msgs = new_buffer.decode_to_array()

        # Validate
        self.assertEqual(metadata.schema, Schema.OHLCV1_S)
        self.assertEqual(metadata.start, 1234567898765)
        self.assertEqual(metadata.end, 123456765432)
        self.assertIsInstance(metadata.mappings, SymbolMap)
        self.assertIsInstance(ohlcv_msgs[0], OhlcvMsg)

    def test_buffer_store_with_metadata(self):
        bin = [
            1,
            141,
            38,
            251,
            113,
            31,
            1,
            0,
            0,
            248,
            189,
            152,
            190,
            28,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            65,
            65,
            80,
            76,
            2,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            84,
            83,
            76,
            65,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            20,
            1,
            0,
            0,
            1,
            0,
            0,
            0,
            212,
            241,
            180,
            96,
            0,
            0,
            0,
            0,
            232,
            3,
            0,
            0,
            0,
            0,
            0,
            0,
            10,
            0,
            0,
            0,
            1,
            1,
            0,
            0,
            13,
            49,
            15,
            134,
            72,
            112,
            0,
            0,
            57,
            48,
            0,
            0,
            64,
            226,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            20,
            1,
            0,
            0,
            1,
            0,
            0,
            0,
            212,
            241,
            180,
            96,
            0,
            0,
            0,
            0,
            232,
            3,
            0,
            0,
            0,
            0,
            0,
            0,
            10,
            0,
            0,
            0,
            1,
            1,
            0,
            0,
            13,
            49,
            15,
            134,
            72,
            112,
            0,
            0,
            57,
            48,
            0,
            0,
            64,
            226,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
        ]

        # Test
        buffer_obj = BufferStore(bytes(bin))
        mbp_msgs = buffer_obj.decode_to_array()

        # Validate
        # Metadata
        self.assertEqual(buffer_obj.metadata.schema, Schema.MBP1)
        self.assertEqual(buffer_obj.metadata.start, 1234567898765)
        self.assertEqual(buffer_obj.metadata.end, 123456765432)
        self.assertIsInstance(buffer_obj.metadata.mappings, SymbolMap)

        # MSG
        self.assertEqual(mbp_msgs[0].hd.instrument_id, 1)
        self.assertEqual(mbp_msgs[0].hd.ts_event, 1622471124)
        self.assertIsInstance(mbp_msgs[0], Mbp1Msg)

    # def test_decode_do_df(self):
    #     # Binary
    #     bin = [
    #         2,
    #         141,
    #         38,
    #         251,
    #         113,
    #         31,
    #         1,
    #         0,
    #         0,
    #         248,
    #         189,
    #         152,
    #         190,
    #         28,
    #         0,
    #         0,
    #         0,
    #         2,
    #         0,
    #         0,
    #         0,
    #         1,
    #         0,
    #         0,
    #         0,
    #         4,
    #         0,
    #         0,
    #         0,
    #         65,
    #         65,
    #         80,
    #         76,
    #         2,
    #         0,
    #         0,
    #         0,
    #         4,
    #         0,
    #         0,
    #         0,
    #         84,
    #         83,
    #         76,
    #         65,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         14,
    #         2,
    #         0,
    #         0,
    #         1,
    #         0,
    #         0,
    #         0,
    #         212,
    #         241,
    #         180,
    #         96,
    #         0,
    #         0,
    #         0,
    #         0,
    #         100,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         200,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         50,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         150,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         232,
    #         3,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         14,
    #         2,
    #         0,
    #         0,
    #         2,
    #         0,
    #         0,
    #         0,
    #         213,
    #         241,
    #         180,
    #         96,
    #         0,
    #         0,
    #         0,
    #         0,
    #         110,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         210,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         55,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         155,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         76,
    #         4,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #         0,
    #     ]

    #     # Test
    #     buffer_obj = BufferStore(bytes(bin))

    #     df = buffer_obj.decode_to_dataframe()
    #     print(df)
    #     # print(buffer_obj.metadata)

    #     # buffer_obj.write_to_file("test.bin")

    #     # new_buffer = BufferStore.from_file("test.bin")
    #     # print(f"Testing {new_buffer}")

    # data = {
    #     "length": [msg.hd.length for msg in ohlcv_msgs],
    #     "rtype": [msg.hd.rtype for msg in ohlcv_msgs],
    #     "instrument_id": [msg.hd.instrument_id for msg in ohlcv_msgs],
    #     "ts_event": [msg.hd.ts_event for msg in ohlcv_msgs],
    #     "open": [msg.open for msg in ohlcv_msgs],
    #     "high": [msg.high for msg in ohlcv_msgs],
    #     "low": [msg.low for msg in ohlcv_msgs],
    #     "close": [msg.close for msg in ohlcv_msgs],
    #     "volume": [msg.volume for msg in ohlcv_msgs],
    # }

    # # Create DataFrame
    # df = pd.DataFrame(data)

    # # Display the DataFrame
    # print(df)

    # print(array[0])


if __name__ == "__main__":
    unittest.main()
