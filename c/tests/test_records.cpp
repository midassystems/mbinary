#include <gtest/gtest.h>

#include <cstdint>
#include <cstring>

#include "mbinary.h"

// Rtype
TEST(RtypeTests, Mbp1) {
  RType mbp1 = RType::Mbp1;

  ASSERT_EQ(mbp1, RType::Mbp1);
  ASSERT_EQ(RType::Mbp1, 1);
}

TEST(RTypeTests, Ohlcv) {
  RType ohlcv = RType::Ohlcv;

  ASSERT_EQ(ohlcv, RType::Ohlcv);
  ASSERT_EQ(RType::Ohlcv, 2);
}

TEST(RTypeTests, Trades) {
  RType trades = RType::Trades;

  ASSERT_EQ(trades, RType::Trades);
  ASSERT_EQ(RType::Trades, 3);
}

TEST(RTypeTests, Tbbo) {
  RType tbbo = RType::Tbbo;

  ASSERT_EQ(tbbo, RType::Tbbo);
  ASSERT_EQ(RType::Tbbo, 4);
}

TEST(RTypeTests, Bbo) {
  RType bbo = RType::Bbo;

  ASSERT_EQ(bbo, RType::Bbo);
  ASSERT_EQ(RType::Bbo, 5);
}

// MBP-1
TEST(Mbp1Tests, ConstructionTest) {
  CRecordEnum record{
      RType::Mbp1,
      RecordData{.mbp1 = {1, RType::Mbp1, 3, 4, 5,  6, 7, 'T', 'A', 1, 2,
                          3, 4,           6, 7, 72, 2, 1, 3,   1,   2}}};

  // RType
  ASSERT_EQ(record.rtype, RType::Mbp1);

  // Header
  const RecordHeader* hd = get_header(&record);
  ASSERT_EQ(hd->length, 1);
  ASSERT_EQ(hd->rtype, RType::Mbp1);
  ASSERT_EQ(hd->instrument_id, 3);
  ASSERT_EQ(hd->ts_event, 4);
  ASSERT_EQ(hd->rollover_flag, 5);

  // Timestamp
  uint64_t ts = get_timestamp(&record);
  ASSERT_EQ(ts, 3);

  // Price
  int price = get_price(&record);
  ASSERT_EQ(price, 6);

  // Record
  const Mbp1Msg* msg = &record.data.mbp1;
  ASSERT_EQ(msg->price, 6);
  ASSERT_EQ(msg->size, 7);
  ASSERT_EQ(msg->action, 84);
  ASSERT_EQ(msg->action, 'T');
  ASSERT_EQ(msg->action, Action::Trade);
  ASSERT_EQ(msg->side, 'A');
  ASSERT_EQ(msg->side, 65);
  ASSERT_EQ(msg->side, Side::Ask);
  ASSERT_EQ(msg->ts_recv, 3);
  ASSERT_EQ(msg->levels[0].ask_px, 2);
  ASSERT_EQ(msg->levels[0].bid_sz, 1);
};

// Tbbo
TEST(TbboTests, ConstructionTest) {
  CRecordEnum record{
      RType::Tbbo,
      RecordData{.tbbo = {1, RType::Tbbo, 3, 4, 5,  6, 7, 'T', 'A', 1, 2,
                          3, 4,           6, 7, 72, 2, 1, 3,   1,   2}}};

  // RType
  ASSERT_EQ(record.rtype, RType::Tbbo);

  // Header
  const RecordHeader* hd = get_header(&record);
  ASSERT_EQ(hd->length, 1);
  ASSERT_EQ(hd->rtype, RType::Tbbo);
  ASSERT_EQ(hd->instrument_id, 3);
  ASSERT_EQ(hd->ts_event, 4);
  ASSERT_EQ(hd->rollover_flag, 5);

  // Timestamp
  uint64_t ts = get_timestamp(&record);
  ASSERT_EQ(ts, 3);

  // Price
  int price = get_price(&record);
  ASSERT_EQ(price, 6);

  // Record
  const TbboMsg* msg = &record.data.tbbo;
  ASSERT_EQ(msg->price, 6);
  ASSERT_EQ(msg->size, 7);
  ASSERT_EQ(msg->action, 84);
  ASSERT_EQ(msg->action, 'T');
  ASSERT_EQ(msg->action, Action::Trade);
  ASSERT_EQ(msg->side, 'A');
  ASSERT_EQ(msg->side, 65);
  ASSERT_EQ(msg->side, Side::Ask);
  ASSERT_EQ(msg->ts_recv, 3);
};

// Trades
TEST(TradeTests, ConstructionTest) {
  CRecordEnum record{RType::Trades, RecordData{.trade = {
                                                   1,
                                                   RType::Trades,
                                                   3,
                                                   4,
                                                   5,
                                                   6,
                                                   7,
                                                   'T',
                                                   'A',
                                                   1,
                                                   2,
                                                   3,
                                                   4,
                                                   6,
                                               }}};

  // RType
  ASSERT_EQ(record.rtype, RType::Trades);

  // Header
  const RecordHeader* hd = get_header(&record);
  ASSERT_EQ(hd->length, 1);
  ASSERT_EQ(hd->rtype, RType::Trades);
  ASSERT_EQ(hd->instrument_id, 3);
  ASSERT_EQ(hd->ts_event, 4);
  ASSERT_EQ(hd->rollover_flag, 5);

  // Timestamp
  uint64_t ts = get_timestamp(&record);
  ASSERT_EQ(ts, 3);

  // Price
  int price = get_price(&record);
  ASSERT_EQ(price, 6);

  // Record
  const TradeMsg* msg = &record.data.trade;
  ASSERT_EQ(msg->price, 6);
  ASSERT_EQ(msg->size, 7);
  ASSERT_EQ(msg->action, 84);
  ASSERT_EQ(msg->action, 'T');
  ASSERT_EQ(msg->action, Action::Trade);
  ASSERT_EQ(msg->side, 'A');
  ASSERT_EQ(msg->side, 65);
  ASSERT_EQ(msg->side, Side::Ask);
  ASSERT_EQ(msg->ts_recv, 3);
};

// Bbo
TEST(BboTests, ConstructionTest) {
  CRecordEnum record{
      RType::Bbo, RecordData{.bbo = {1, RType::Bbo, 3, 4, 5, 6, 7, 'A', 0, 1, 2,
                                     3, 4, 6, 7, 72, 2}}};

  // RType
  ASSERT_EQ(record.rtype, RType::Bbo);

  // Header
  const RecordHeader* hd = get_header(&record);
  ASSERT_EQ(hd->length, 1);
  ASSERT_EQ(hd->rtype, RType::Bbo);
  ASSERT_EQ(hd->instrument_id, 3);
  ASSERT_EQ(hd->ts_event, 4);
  ASSERT_EQ(hd->rollover_flag, 5);

  // Timestamp
  uint64_t ts = get_timestamp(&record);
  ASSERT_EQ(ts, 1);

  // Price
  int price = get_price(&record);
  ASSERT_EQ(price, 6);

  // Record
  const BboMsg* msg = &record.data.bbo;
  ASSERT_EQ(msg->price, 6);
  ASSERT_EQ(msg->size, 7);
  ASSERT_EQ(msg->side, 'A');
  ASSERT_EQ(msg->side, 65);
  ASSERT_EQ(msg->side, Side::Ask);
  ASSERT_EQ(msg->ts_recv, 1);
  ASSERT_EQ(msg->levels[0].ask_px, 4);
  ASSERT_EQ(msg->levels[0].bid_sz, 6);
};

// Ohlcv
TEST(OhlcvTests, ConstructionTest) {
  CRecordEnum record{RType::Ohlcv, RecordData{.ohlcv = {1, RType::Ohlcv, 3, 4,
                                                        5, 6, 7, 8, 9, 10}}};

  // RType
  ASSERT_EQ(record.rtype, RType::Ohlcv);

  // Header
  const RecordHeader* hd = get_header(&record);
  ASSERT_EQ(hd->length, 1);
  ASSERT_EQ(hd->rtype, RType::Ohlcv);
  ASSERT_EQ(hd->instrument_id, 3);
  ASSERT_EQ(hd->ts_event, 4);
  ASSERT_EQ(hd->rollover_flag, 5);

  // Timestamp
  uint64_t ts = get_timestamp(&record);
  ASSERT_EQ(ts, 4);

  // Price
  int price = get_price(&record);
  ASSERT_EQ(price, 9);

  // Record
  const OhlcvMsg* msg = &record.data.ohlcv;
  ASSERT_EQ(msg->open, 6);
  ASSERT_EQ(msg->high, 7);
  ASSERT_EQ(msg->low, 8);
  ASSERT_EQ(msg->close, 9);
  ASSERT_EQ(msg->volume, 10);
};
