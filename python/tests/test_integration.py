import unittest
import json
from mbinary import (
    Side,
    Action,
    Schema,
    RType,
    SymbolMap,
    Metadata,
    BufferStore,
    BidAskPair,
    RecordMsg,
    OhlcvMsg,
    Mbp1Msg,
    TradeMsg,
    BboMsg,
    BacktestData,
    BacktestMetaData,
    Trades,
    Signals,
    Parameters,
    TimeseriesStats,
    StaticStats,
    SignalInstructions,
    AccountSummary,
    LiveData,
    PyRecordEncoder,
    PyMetadataEncoder,
    Vendors,
    Dataset,
    Stype,
    RetrieveParams,
)
import mbinary
from pandas import pandas


def handle_msg(msg: RecordMsg) -> int:
    return msg.ts_event


def read_file_into_buffer(file_path):
    with open(file_path, "rb") as file:  # Open the file in binary read mode
        buffer = file.read()  # Read the entire file into a buffer
    return buffer


class IntegrationTests(unittest.TestCase):
    def setUp(self) -> None:

        self.parameters = Parameters(
            strategy_name="Testing",
            capital=10000,
            schema="Ohlcv-1s",
            data_type="BAR",
            start=1730160814000000000,
            end=1730160814000000000,
            tickers=["test", "test2"],
        )

        self.static_stats = StaticStats(
            total_trades=100,
            total_winning_trades=50,
            total_losing_trades=50,
            avg_profit=1000000000000,
            avg_profit_percent=10383783337737,
            avg_gain=23323212233,
            avg_gain_percent=24323234,
            avg_loss=203982828,
            avg_loss_percent=23432134323,
            profitability_ratio=130213212323,
            profit_factor=12342123431,
            profit_and_loss_ratio=1234321343,
            total_fees=123453234,
            net_profit=1234323,
            beginning_equity=12343234323,
            ending_equity=12343234,
            total_return=234532345,
            annualized_return=234532345,
            daily_standard_deviation_percentage=23453234,
            annual_standard_deviation_percentage=34543443,
            max_drawdown_percentage_period=234543234,
            max_drawdown_percentage_daily=23432345,
            sharpe_ratio=23432343,
            sortino_ratio=123453234543,
        )

        # Test
        self.timeseries1 = TimeseriesStats(
            timestamp=123700000000000,
            equity_value=9999999,
            percent_drawdown=2343234,
            cumulative_return=2343234,
            period_return=2345432345,
        )

        self.timeseries2 = TimeseriesStats(
            timestamp=123700000000000,
            equity_value=9999999,
            percent_drawdown=2343234,
            cumulative_return=2343234,
            period_return=2345432345,
        )
        self.period_list = [self.timeseries1, self.timeseries2]

        self.timeseries3 = TimeseriesStats(
            timestamp=123700000000000,
            equity_value=9999999,
            percent_drawdown=2343234,
            cumulative_return=2343234,
            period_return=2345432345,
        )

        self.timeseries4 = TimeseriesStats(
            timestamp=123700000000000,
            equity_value=9999999,
            percent_drawdown=2343234,
            cumulative_return=2343234,
            period_return=2345432345,
        )
        self.daily_list = [self.timeseries3, self.timeseries4]

        self.trade1 = Trades(
            trade_id=1,
            signal_id=2,
            timestamp=170000000000,
            ticker="AAPL",
            quantity=12,
            avg_price=2345432,
            trade_value=12343234,
            trade_cost=12343234,
            action="BUY",
            fees=2343,
        )

        self.trade2 = Trades(
            trade_id=1,
            signal_id=2,
            timestamp=170000000000,
            ticker="AAPL",
            quantity=12,
            avg_price=2345432,
            trade_value=12343234,
            trade_cost=12343234,
            action="BUY",
            fees=2343,
        )

        self.trade_list = [self.trade1, self.trade2]

        self.instructions = SignalInstructions(
            ticker="AAPL",
            order_type="MKT",
            action="BUY",
            signal_id=1,
            weight=13213432,
            quantity=2343,
            limit_price="",
            aux_price="",
        )

        self.signal1 = Signals(1234532345, [self.instructions])
        self.signal2 = Signals(1234532345, [self.instructions])
        self.signals_list = [self.signal1, self.signal2]

        self.account_summary = AccountSummary(
            currency="USD",
            start_timestamp=1704903000,
            start_buying_power=2557567,
            start_excess_liquidity=767270,
            start_full_available_funds=767270,
            start_full_init_margin_req=2823937,
            start_full_maint_margin_req=2823938,
            start_futures_pnl=-464883,
            start_net_liquidation=767552392,
            start_total_cash_balance=-11292332,
            start_unrealized_pnl=0,
            end_timestamp=1704904000,
            end_buying_power=25355889282,
            end_excess_liquidity=762034292,
            end_full_available_funds=760676292,
            end_full_init_margin_req=707499,
            end_full_maint_margin_req=5716009,
            end_futures_pnl=-487998,
            end_net_liquidation=767751998,
            end_total_cash_balance=76693599,
            end_unrealized_pnl=-2899,
        )
        return super().setUp()

    # -- Live --
    def test_live_data(self):
        # Test
        live = LiveData(
            None,
            self.parameters,
            self.trade_list,
            self.signals_list,
            self.account_summary,
        )

        # Validate
        expected = {
            "live_id": None,
            "parameters": {
                "strategy_name": "Testing",
                "capital": 10000,
                "schema": "Ohlcv-1s",
                "data_type": "BAR",
                "start": 1730160814000000000,
                "end": 1730160814000000000,
                "tickers": ["test", "test2"],
            },
            "account": {
                "currency": "USD",
                "start_timestamp": 1704903000,
                "start_buying_power": 2557567,
                "start_excess_liquidity": 767270,
                "start_full_available_funds": 767270,
                "start_full_init_margin_req": 2823937,
                "start_full_maint_margin_req": 2823938,
                "start_futures_pnl": -464883,
                "start_net_liquidation": 767552392,
                "start_total_cash_balance": -11292332,
                "start_unrealized_pnl": 0,
                "end_timestamp": 1704904000,
                "end_buying_power": 25355889282,
                "end_excess_liquidity": 762034292,
                "end_full_available_funds": 760676292,
                "end_full_init_margin_req": 707499,
                "end_full_maint_margin_req": 5716009,
                "end_futures_pnl": -487998,
                "end_net_liquidation": 767751998,
                "end_total_cash_balance": 76693599,
                "end_unrealized_pnl": -2899,
            },
            "trades": [
                {
                    "trade_id": 1,
                    "signal_id": 2,
                    "timestamp": 170000000000,
                    "ticker": "AAPL",
                    "quantity": 12,
                    "avg_price": 2345432,
                    "trade_value": 12343234,
                    "trade_cost": 12343234,
                    "action": "BUY",
                    "fees": 2343,
                },
                {
                    "trade_id": 1,
                    "signal_id": 2,
                    "timestamp": 170000000000,
                    "ticker": "AAPL",
                    "quantity": 12,
                    "avg_price": 2345432,
                    "trade_value": 12343234,
                    "trade_cost": 12343234,
                    "action": "BUY",
                    "fees": 2343,
                },
            ],
            "signals": [
                {
                    "timestamp": 1234532345,
                    "trade_instructions": [
                        {
                            "ticker": "AAPL",
                            "order_type": "MKT",
                            "action": "BUY",
                            "signal_id": 1,
                            "weight": 13213432,
                            "quantity": 2343,
                            "limit_price": "",
                            "aux_price": "",
                        }
                    ],
                },
                {
                    "timestamp": 1234532345,
                    "trade_instructions": [
                        {
                            "ticker": "AAPL",
                            "order_type": "MKT",
                            "action": "BUY",
                            "signal_id": 1,
                            "weight": 13213432,
                            "quantity": 2343,
                            "limit_price": "",
                            "aux_price": "",
                        }
                    ],
                },
            ],
        }
        self.assertDictEqual(expected, live.to_dict())

    # -- Backtest --
    def test_backtest_data(self):
        metadata = BacktestMetaData(
            1234,
            "Name",
            self.parameters,
            self.static_stats,
        )

        # Test
        backtest = BacktestData(
            metadata,
            self.period_list,
            self.daily_list,
            self.trade_list,
            self.signals_list,
        )

        # Validate
        expected = {
            "metadata": {
                "backtest_id": 1234,
                "backtest_name": "Name",
                "parameters": {
                    "strategy_name": "Testing",
                    "capital": 10000,
                    "schema": "Ohlcv-1s",
                    "data_type": "BAR",
                    "start": 1730160814000000000,
                    "end": 1730160814000000000,
                    "tickers": ["test", "test2"],
                },
                "static_stats": {
                    "total_trades": 100,
                    "total_winning_trades": 50,
                    "total_losing_trades": 50,
                    "avg_profit": 1000000000000,
                    "avg_profit_percent": 10383783337737,
                    "avg_gain": 23323212233,
                    "avg_gain_percent": 24323234,
                    "avg_loss": 203982828,
                    "avg_loss_percent": 23432134323,
                    "profitability_ratio": 130213212323,
                    "profit_factor": 12342123431,
                    "profit_and_loss_ratio": 1234321343,
                    "total_fees": 123453234,
                    "net_profit": 1234323,
                    "beginning_equity": 12343234323,
                    "ending_equity": 12343234,
                    "total_return": 234532345,
                    "annualized_return": 234532345,
                    "daily_standard_deviation_percentage": 23453234,
                    "annual_standard_deviation_percentage": 34543443,
                    "max_drawdown_percentage_daily": 23432345,
                    "max_drawdown_percentage_period": 234543234,
                    "sharpe_ratio": 23432343,
                    "sortino_ratio": 123453234543,
                },
            },
            "period_timeseries_stats": [
                {
                    "timestamp": 123700000000000,
                    "equity_value": 9999999,
                    "percent_drawdown": 2343234,
                    "period_return": 2345432345,
                    "cumulative_return": 2343234,
                },
                {
                    "timestamp": 123700000000000,
                    "equity_value": 9999999,
                    "percent_drawdown": 2343234,
                    "period_return": 2345432345,
                    "cumulative_return": 2343234,
                },
            ],
            "daily_timeseries_stats": [
                {
                    "timestamp": 123700000000000,
                    "equity_value": 9999999,
                    "percent_drawdown": 2343234,
                    "period_return": 2345432345,
                    "cumulative_return": 2343234,
                },
                {
                    "timestamp": 123700000000000,
                    "equity_value": 9999999,
                    "percent_drawdown": 2343234,
                    "period_return": 2345432345,
                    "cumulative_return": 2343234,
                },
            ],
            "trades": [
                {
                    "trade_id": 1,
                    "signal_id": 2,
                    "timestamp": 170000000000,
                    "ticker": "AAPL",
                    "quantity": 12,
                    "avg_price": 2345432,
                    "trade_value": 12343234,
                    "trade_cost": 12343234,
                    "action": "BUY",
                    "fees": 2343,
                },
                {
                    "trade_id": 1,
                    "signal_id": 2,
                    "timestamp": 170000000000,
                    "ticker": "AAPL",
                    "quantity": 12,
                    "avg_price": 2345432,
                    "trade_value": 12343234,
                    "trade_cost": 12343234,
                    "action": "BUY",
                    "fees": 2343,
                },
            ],
            "signals": [
                {
                    "timestamp": 1234532345,
                    "trade_instructions": [
                        {
                            "ticker": "AAPL",
                            "order_type": "MKT",
                            "action": "BUY",
                            "signal_id": 1,
                            "weight": 13213432,
                            "quantity": 2343,
                            "limit_price": "",
                            "aux_price": "",
                        }
                    ],
                },
                {
                    "timestamp": 1234532345,
                    "trade_instructions": [
                        {
                            "ticker": "AAPL",
                            "order_type": "MKT",
                            "action": "BUY",
                            "signal_id": 1,
                            "weight": 13213432,
                            "quantity": 2343,
                            "limit_price": "",
                            "aux_price": "",
                        }
                    ],
                },
            ],
        }
        self.assertEqual(expected, backtest.to_dict())

    def test_parameters(self):
        strategy_name = "Testing"
        capital = 10000
        schema = "Ohlcv-1s"
        data_type = "BAR"
        start = 1730160814000000000
        end = 1730160814000000000
        tickers = ["test", "test2"]

        # Test
        parameters = Parameters(
            strategy_name,
            capital,
            schema,
            data_type,
            start,
            end,
            tickers,
        )

        # Validate
        expected = {
            "strategy_name": "Testing",
            "capital": 10000,
            "schema": "Ohlcv-1s",
            "data_type": "BAR",
            "start": 1730160814000000000,
            "end": 1730160814000000000,
            "tickers": ["test", "test2"],
        }
        self.assertEqual(expected, parameters.to_dict())

    def test_static_stats(self):
        total_trades = 100
        total_winning_trades = 50
        total_losing_trades = 50
        avg_profit = 1000000000000
        avg_profit_percent = 10383783337737
        avg_gain = 23323212233
        avg_gain_percent = 24323234
        avg_loss = 203982828
        avg_loss_percent = 23432134323
        profitability_ratio = 130213212323
        profit_factor = 12342123431
        profit_and_loss_ratio = 1234321343
        total_fees = 123453234
        net_profit = 1234323
        beginning_equity = 12343234323
        ending_equity = 12343234
        total_return = 234532345
        annualized_return = 234532345
        daily_standard_deviation_percentage = 23453234
        annual_standard_deviation_percentage = 34543443
        max_drawdown_percentage_period = 234543234
        max_drawdown_percentage_daily = 23432345
        sharpe_ratio = 23432343
        sortino_ratio = 123453234543

        # Test
        static_stats = StaticStats(
            total_trades,
            total_winning_trades,
            total_losing_trades,
            avg_profit,
            avg_profit_percent,
            avg_gain,
            avg_gain_percent,
            avg_loss,
            avg_loss_percent,
            profitability_ratio,
            profit_factor,
            profit_and_loss_ratio,
            total_fees,
            net_profit,
            beginning_equity,
            ending_equity,
            total_return,
            annualized_return,
            daily_standard_deviation_percentage,
            annual_standard_deviation_percentage,
            max_drawdown_percentage_period,
            max_drawdown_percentage_daily,
            sharpe_ratio,
            sortino_ratio,
        )

        # Validate
        expected = {
            "total_trades": 100,
            "total_winning_trades": 50,
            "total_losing_trades": 50,
            "avg_profit": 1000000000000,
            "avg_profit_percent": 10383783337737,
            "avg_gain": 23323212233,
            "avg_gain_percent": 24323234,
            "avg_loss": 203982828,
            "avg_loss_percent": 23432134323,
            "profitability_ratio": 130213212323,
            "profit_factor": 12342123431,
            "profit_and_loss_ratio": 1234321343,
            "total_fees": 123453234,
            "net_profit": 1234323,
            "beginning_equity": 12343234323,
            "ending_equity": 12343234,
            "total_return": 234532345,
            "annualized_return": 234532345,
            "daily_standard_deviation_percentage": 23453234,
            "annual_standard_deviation_percentage": 34543443,
            "max_drawdown_percentage_daily": 23432345,
            "max_drawdown_percentage_period": 234543234,
            "sharpe_ratio": 23432343,
            "sortino_ratio": 123453234543,
        }
        self.assertEqual(expected, static_stats.to_dict())

    def test_constants(self):
        self.assertEqual(mbinary.PRICE_SCALE, 1000000000)
        self.assertEqual(mbinary.QUANTITY_SCALE, 1000)

    def test_timeseries(self):
        timestamp = 123700000000000
        equity_value = 9999999
        percent_drawdown = 2343234
        cumulative_return = 2343234
        period_return = 2345432345

        # Test
        timeseries = TimeseriesStats(
            timestamp,
            equity_value,
            percent_drawdown,
            cumulative_return,
            period_return,
        )

        # Validate
        expected = {
            "timestamp": 123700000000000,
            "equity_value": 9999999,
            "percent_drawdown": 2343234,
            "period_return": 2345432345,
            "cumulative_return": 2343234,
        }
        self.assertEqual(expected, timeseries.to_dict())

    def test_trades(self):
        trade_id = 1
        signal_id = 2
        timestamp = 170000000000
        ticker = "AAPL"
        quantity = 12
        avg_price = 2345432
        trade_value = 12343234
        trade_cost = 12343234
        action = "BUY"
        fees = 2343

        # Test
        trade = Trades(
            trade_id,
            signal_id,
            timestamp,
            ticker,
            quantity,
            avg_price,
            trade_value,
            trade_cost,
            action,
            fees,
        )

        # Validate
        expected = {
            "trade_id": 1,
            "signal_id": 2,
            "timestamp": 170000000000,
            "ticker": "AAPL",
            "quantity": 12,
            "avg_price": 2345432,
            "trade_value": 12343234,
            "trade_cost": 12343234,
            "action": "BUY",
            "fees": 2343,
        }
        self.assertEqual(expected, trade.to_dict())

    def test_signal(self):
        ticker = "AAPL"
        order_type = "MKT"
        action = "BUY"
        signal_id = 1
        weight = 13213432
        quantity = 2343
        limit_price = ""
        aux_price = ""
        timestamp = 1234532345

        instructions = SignalInstructions(
            ticker,
            order_type,
            action,
            signal_id,
            weight,
            quantity,
            limit_price,
            aux_price,
        )

        # Test
        signal = Signals(timestamp, [instructions])

        # Validate
        expected = {
            "timestamp": 1234532345,
            "trade_instructions": [
                {
                    "ticker": "AAPL",
                    "order_type": "MKT",
                    "action": "BUY",
                    "signal_id": 1,
                    "weight": 13213432,
                    "quantity": 2343,
                    "limit_price": "",
                    "aux_price": "",
                }
            ],
        }
        self.assertEqual(expected, signal.to_dict())

    def test_signalinstructions(self):
        ticker = "AAPL"
        order_type = "MKT"
        action = "BUY"
        signal_id = 1
        weight = 13213432
        quantity = 2343
        limit_price = ""
        aux_price = ""

        # Test
        instructions = SignalInstructions(
            ticker,
            order_type,
            action,
            signal_id,
            weight,
            quantity,
            limit_price,
            aux_price,
        )

        # Validate
        expected = {
            "ticker": "AAPL",
            "order_type": "MKT",
            "action": "BUY",
            "signal_id": 1,
            "weight": 13213432,
            "quantity": 2343,
            "limit_price": "",
            "aux_price": "",
        }
        self.assertEqual(expected, instructions.to_dict())

    # -- Records --
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

        # valure
        act_1 = Action.MODIFY
        act_2 = Action.from_str("M")
        self.assertEqual(act_1, act_2)

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

        # valure
        act_1 = Schema.MBP1
        act_2 = Schema.from_str("mbp-1")
        self.assertEqual(act_1, act_2)

        self.assertEqual(act_1.value, "mbp-1")
        self.assertEqual(act_2.value, "mbp-1")

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

        # valure
        act_1 = RType.MBP1
        act_2 = RType.from_str("mbp-1")
        self.assertEqual(act_1, act_2)

        self.assertEqual(act_1.value, "mbp-1")
        self.assertEqual(act_2.value, "mbp-1")
        # Errors
        with self.assertRaises(ValueError):
            RType.from_int(0x09)

        with self.assertRaises(ValueError):
            RType.from_str("olghd")

    def test_vendors(self):
        # instantiation
        db = Vendors.DATABENTO
        self.assertEqual(db, Vendors.DATABENTO)

        # from str
        db = Vendors.from_str("databento")
        self.assertEqual(db, Vendors.DATABENTO)

        # __str__
        v = Vendors.YFINANCE.__str__()
        self.assertEqual(v, "yfinance")

        # valure
        act_1 = Vendors.DATABENTO
        act_2 = Vendors.from_str("databento")
        self.assertEqual(act_1, act_2)

        self.assertEqual(act_1.value, "databento")
        self.assertEqual(act_2.to_json(), "Databento")

        # Error
        with self.assertRaises(ValueError):
            Vendors.from_str("ohlcv-12345s")

    def test_dataset(self):
        # instantiation
        d = Dataset.EQUITIES
        self.assertEqual(d, Dataset.EQUITIES)

        # from str
        d = Dataset.from_str("futures")
        self.assertEqual(d, Dataset.FUTURES)

        # __str__
        d = Dataset.OPTION.__str__()
        self.assertEqual(d, "option")

        # Error
        with self.assertRaises(ValueError):
            Dataset.from_str("ohlcv-12345s")

    def test_stype(self):
        # instantiation
        s = Stype.CONTINUOUS
        self.assertEqual(s, Stype.CONTINUOUS)

        # from str
        s = Stype.from_str("raw")
        self.assertEqual(s, Stype.RAW)

        # __str__
        s = Stype.CONTINUOUS.__str__()
        self.assertEqual(s, "continuous")

        # Error
        with self.assertRaises(ValueError):
            Stype.from_str("ohlcv-12345s")

    def test_metadata(self):
        symbol_map = SymbolMap({1: "AAPL", 2: "TSLA"})

        # Test
        metadata = Metadata(
            Schema.from_str("ohlcv-1s"),
            Dataset.from_str("equities"),
            1234567654321,
            987654345676543456,
            symbol_map,
        )
        encoded = metadata.encode()
        decoded_metadata = metadata.decode(encoded)

        # Validate
        self.assertEqual(decoded_metadata.start, metadata.start)
        self.assertEqual(decoded_metadata.schema, metadata.schema)
        self.assertEqual(decoded_metadata.dataset, metadata.dataset)
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

    def test_is_record(self):
        pair = BidAskPair(1, 2, 3, 4, 5, 6)
        ohlcv_msg = OhlcvMsg(1, 123456765432, 0, 1, 2, 3, 4, 100000)
        mbp1_msg = Mbp1Msg(
            1,
            123456765432,
            0,
            1,
            2,
            Action.ADD,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
            0,
            [pair],
        )
        trade_msg = TradeMsg(
            1,
            123456765432,
            0,
            1,
            2,
            Action.TRADE,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
        )
        bbo_msg = BboMsg(
            1,
            123456765432,
            0,
            [pair],
        )

        # Test
        self.assertTrue(RecordMsg.is_record(ohlcv_msg))
        self.assertTrue(RecordMsg.is_record(mbp1_msg))
        self.assertTrue(RecordMsg.is_record(bbo_msg))
        self.assertTrue(RecordMsg.is_record(trade_msg))

    def test_bid_ask_properties(self):
        pair = BidAskPair(1, 2, 3, 4, 5, 6)

        # Validate
        self.assertEqual(pair.bid_px, 1)
        self.assertEqual(pair.ask_px, 2)
        self.assertEqual(pair.bid_sz, 3)
        self.assertEqual(pair.ask_sz, 4)
        self.assertEqual(pair.bid_ct, 5)
        self.assertEqual(pair.ask_ct, 6)
        self.assertEqual(pair.pretty_bid_px, 1 / 1e9)
        self.assertEqual(pair.pretty_ask_px, 2 / 1e9)

    def test_ohlcvmsg_properties(self):
        msg = OhlcvMsg(1, 123456765432, 0, 1, 2, 3, 4, 100000)

        # Test
        self.assertEqual(msg.rtype, RType.OHLCV)
        self.assertEqual(msg.instrument_id, 1)
        self.assertEqual(msg.ts_event, 123456765432)
        self.assertEqual(msg.ts, 123456765432)
        self.assertEqual(msg.rollover_flag, 0)
        self.assertEqual(msg.open, 1)
        self.assertEqual(msg.pretty_open, 1 / 1e9)
        self.assertEqual(msg.high, 2)
        self.assertEqual(msg.pretty_high, 2 / 1e9)
        self.assertEqual(msg.low, 3)
        self.assertEqual(msg.pretty_low, 3 / 1e9)
        self.assertEqual(msg.close, 4)
        self.assertEqual(msg.pretty_close, 4 / 1e9)
        self.assertEqual(msg.volume, 100000)
        self.assertEqual(msg.pretty_price, 4 / 1e9)
        # self.assert(msg, RecordMsg)

    def test_mbpmsg_properties(self):
        pair = BidAskPair(1, 2, 3, 4, 5, 6)
        msg = Mbp1Msg(
            1,
            123456765432,
            0,
            1,
            2,
            Action.ADD,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
            0,
            [pair],
        )
        msg.instrument_id = 2

        # Test
        self.assertEqual(msg.rtype, RType.MBP1)
        self.assertEqual(msg.instrument_id, 2)
        self.assertEqual(msg.ts, 3)
        self.assertEqual(msg.ts_event, 123456765432)
        self.assertEqual(msg.rollover_flag, 0)
        self.assertEqual(msg.price, 1)
        self.assertEqual(msg.pretty_price, 1 / 1e9)
        self.assertEqual(msg.action, 65)
        self.assertEqual(msg.pretty_action, Action.ADD)
        self.assertEqual(msg.pretty_side, Side.ASK)
        self.assertEqual(msg.side, 65)
        self.assertEqual(msg.depth, 0)
        self.assertEqual(msg.ts_recv, 3)
        self.assertEqual(msg.ts_in_delta, 4)
        self.assertEqual(msg.sequence, 5)
        self.assertEqual(msg.discriminator, 0)
        self.assertEqual(msg.levels[0].bid_px, pair.bid_px)
        self.assertEqual(msg.levels[0].ask_px, pair.ask_px)
        self.assertEqual(msg.levels[0].bid_sz, pair.bid_sz)
        self.assertEqual(msg.levels[0].ask_sz, pair.ask_sz)
        self.assertEqual(msg.levels[0].bid_ct, pair.bid_ct)
        self.assertEqual(msg.levels[0].ask_ct, pair.ask_ct)

    def test_trademsg_properties(self):
        msg = TradeMsg(
            1,
            123456765432,
            0,
            1,
            2,
            Action.TRADE,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
        )

        # Test
        self.assertEqual(msg.rtype, RType.TRADES)
        self.assertEqual(msg.instrument_id, 1)
        self.assertEqual(msg.ts_event, 123456765432)
        self.assertEqual(msg.ts, 3)
        self.assertEqual(msg.rollover_flag, 0)
        self.assertEqual(msg.price, 1)
        self.assertEqual(msg.pretty_price, 1 / 1e9)
        self.assertEqual(msg.action, 84)
        self.assertEqual(msg.pretty_action, Action.TRADE)
        self.assertEqual(msg.pretty_side, Side.ASK)
        self.assertEqual(msg.side, 65)
        self.assertEqual(msg.depth, 0)
        self.assertEqual(msg.ts_recv, 3)
        self.assertEqual(msg.ts_in_delta, 4)
        self.assertEqual(msg.sequence, 5)

    def test_bbomsg_properties(self):
        pair = BidAskPair(1000000000, 2000000000, 3, 4, 5, 6)
        msg = BboMsg(
            1,
            123456765432,
            0,
            [pair],
        )

        # Test
        self.assertEqual(msg.rtype, RType.BBO)
        self.assertEqual(msg.instrument_id, 1)
        self.assertEqual(msg.ts_event, 123456765432)
        self.assertEqual(msg.ts, 123456765432)
        self.assertEqual(msg.rollover_flag, 0)
        self.assertEqual(msg.pretty_price, 1.5)
        self.assertEqual(msg.price, 1500000000)
        self.assertEqual(msg.levels[0].bid_px, pair.bid_px)
        self.assertEqual(msg.levels[0].ask_px, pair.ask_px)
        self.assertEqual(msg.levels[0].bid_sz, pair.bid_sz)
        self.assertEqual(msg.levels[0].ask_sz, pair.ask_sz)
        self.assertEqual(msg.levels[0].bid_ct, pair.bid_ct)
        self.assertEqual(msg.levels[0].ask_ct, pair.ask_ct)

    def test_bbomsg_updated(self):
        pair = BidAskPair(1, 2, 3, 4, 5, 6)
        msg = BboMsg(
            1,
            123456765432,
            0,
            [pair],
        )
        pair2 = BidAskPair(1234, 1234, 1234, 1234, 1234, 1234)
        msg.levels = [pair2]  # type: ignore

        # Test
        self.assertEqual(msg.levels[0].bid_px, 1234)
        self.assertEqual(msg.levels[0].ask_px, 1234)
        self.assertEqual(msg.levels[0].bid_sz, 1234)
        self.assertEqual(msg.levels[0].ask_sz, 1234)
        self.assertEqual(msg.levels[0].bid_ct, 1234)
        self.assertEqual(msg.levels[0].ask_ct, 1234)

    def test_msg_polymorphism(self):
        msg = OhlcvMsg(1, 123456765432, 1, 1, 2, 3, 4, 100000)

        # Test
        ts_event = handle_msg(msg)
        self.assertEqual(ts_event, msg.ts_event)

    def test_encode_records(self):
        bin = []

        # Metadata
        symbol_map = SymbolMap({})
        metadata = Metadata(
            Schema.from_str("mbp-1"),
            Dataset.from_str("equities"),
            1234567654321,
            987654345676543456,
            symbol_map,
        )

        encoder = PyMetadataEncoder()
        encoder.encode_metadata(metadata)
        binary = encoder.get_encoded_data()
        bin.extend(binary)

        # Record
        pair = BidAskPair(1, 2, 3, 4, 5, 6)
        msg = Mbp1Msg(
            1,
            123456765432,
            1,
            1,
            2,
            Action.ADD,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
            0,
            [pair],
        )

        encoder = PyRecordEncoder()
        encoder.encode_records([msg])
        binary = encoder.get_encoded_data()
        bin.extend(binary)

        self.assertTrue(len(bin) > 0)

    def test_encode(self):
        pair = BidAskPair(1, 2, 3, 4, 5, 6)
        msg = Mbp1Msg(
            1,
            123456765432,
            1,
            1,
            2,
            Action.ADD,
            Side.ASK,
            0,
            0,
            3,
            4,
            5,
            0,
            [pair],
        )

        encoder = PyRecordEncoder()
        encoder.encode_records([msg])
        binary = encoder.get_encoded_data()

        self.assertTrue(len(binary) > 0)

    def test_retrieve_params_json(self):
        # Create an instance
        params = RetrieveParams(
            symbols=["AAPL", "GOOG"],
            start="2024-01-01",
            end="2024-01-02",
            schema=Schema.MBP1,
            dataset=Dataset.EQUITIES,
            stype=Stype.RAW,
        )

        # Serialize to JSON
        json_str = params.to_json()

        # Validate
        expected = json.dumps(
            {
                "symbols": ["AAPL", "GOOG"],
                "start_ts": 1704067200000000000,
                "end_ts": 1704153600000000000,
                "schema": "Mbp1",
                "dataset": "Equities",
                "stype": "Raw",
            }
        )
        self.assertEqual(json.loads(expected), json.loads(json_str))

    def test_buffer_store_to_file(self):

        # Usage
        file_path = "tests/mbp_w_metadata.bin"  # Replace with your file path
        buffer = read_file_into_buffer(file_path)

        # Convert buffer to a list of integers for demonstration
        bin = list(buffer)

        # Write bin file
        buffer_obj = BufferStore(bytes(bin))
        buffer_obj.write_to_file("tests/test.bin")

        # Test
        new_buffer = BufferStore.from_file("tests/test.bin")
        metadata = new_buffer.metadata
        msgs = new_buffer.decode_to_array()

        # Validate
        self.assertEqual(metadata.schema, Schema.MBP1)
        self.assertEqual(metadata.start, 1234567898765)
        self.assertEqual(metadata.end, 123456765432)
        self.assertIsInstance(metadata.mappings, SymbolMap)
        self.assertIsInstance(msgs[0], Mbp1Msg)

    def test_buffer_store_with_metadata(self):
        file_path = "tests/mbp_w_metadata.bin"
        buffer = read_file_into_buffer(file_path)

        # Convert buffer to a list of integers for demonstration
        bin = list(buffer)

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

    def test_decode_do_df(self):
        file_path = "tests/mbp_w_metadata.bin"  # Replace with your file path
        buffer = read_file_into_buffer(file_path)

        # Convert buffer to a list of integers for demonstration
        bin = list(buffer)

        # Test
        buffer_obj = BufferStore(bytes(bin))
        df = buffer_obj.decode_to_df(pretty_ts=True, pretty_px=False)

        # Valdiate
        self.assertIsInstance(df, pandas.DataFrame)

    def test_decode_replay(self):
        file_path = "tests/mbp_w_metadata.bin"
        buffer = read_file_into_buffer(file_path)

        # Convert buffer to a list of integers for demonstration
        bin = list(buffer)

        # Test
        buffer_obj = BufferStore(bytes(bin))

        record = buffer_obj.replay()
        ts_event = 0
        while record is not None:
            self.assertTrue(record.ts_event > ts_event)
            record = buffer_obj.replay()


if __name__ == "__main__":
    unittest.main()
