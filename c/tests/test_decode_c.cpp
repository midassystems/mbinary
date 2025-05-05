#include <gtest/gtest.h>

#include <cstdint>
#include <cstdio>
#include <cstring>
#include <vector>

#include "mbinary.h"

TEST(DecoderTests, test_decode_buffer) {
  // Prepare test records
  uint8_t length = sizeof(Mbp1Msg) / METADATA_LENGTH_MULTIPLIER;
  CRecordEnum record1{
      .rtype = RType::Mbp1,
      .data = RecordData{
          .mbp1 = {length, RType::Mbp1, 1,   1622471124, 0, 1000,
                   10,     'T',         'A', 0,          0, 123456789098765,
                   12345,  123456,      0,   1,          2, 2,
                   2,      1,           3}}};

  std::vector<RecordData> records;
  records.push_back(record1.data);

  // Create the CRecordEncoder
  CRecordEncoder* encoder = create_record_encoder();
  ASSERT_NE(encoder, nullptr) << "Failed to create CRecordEncoder";

  // Encode the records
  int result = encode_records(encoder, records.data(), records.size());
  EXPECT_EQ(result, 0) << "Failed to encode records";

  // Retrieve encoded data
  size_t encoded_size = 0;
  get_encoded_data(encoder, nullptr, &encoded_size);  // Query size first
  ASSERT_GT(encoded_size, 0) << "Encoded data size is zero";

  std::vector<uint8_t> encoded_data(encoded_size);
  get_encoded_data(encoder, encoded_data.data(), &encoded_size);

  // Destroy the encoder
  destroy_record_encoder(encoder);

  // Test
  CRecordDecoder* decoder =
      create_buffer_decoder(encoded_data.data(), encoded_size);

  CRecordEnum record;
  while (decoder_iter(decoder, &record)) {
    const Mbp1Msg* msg = &record.data.mbp1;
    ASSERT_EQ(msg->hd.rtype, RType::Mbp1);
    ASSERT_EQ(msg->hd.instrument_id, 1);
    ASSERT_EQ(msg->hd.rollover_flag, 0);
    ASSERT_EQ(msg->price, 1000);
    ASSERT_EQ(msg->size, 10);
    ASSERT_EQ(msg->action, Action::Trade);
    ASSERT_EQ(msg->side, Side::Ask);
    ASSERT_EQ(msg->depth, 0);
    ASSERT_EQ(msg->flags, 0);
    ASSERT_EQ(msg->ts_recv, 123456789098765);
    ASSERT_EQ(msg->ts_in_delta, 12345);
    ASSERT_EQ(msg->sequence, 123456);
    ASSERT_EQ(msg->discriminator, 0);
    ASSERT_EQ(msg->levels[0].bid_px, 1);
    ASSERT_EQ(msg->levels[0].ask_px, 2);
    ASSERT_EQ(msg->levels[0].bid_sz, 2);
    ASSERT_EQ(msg->levels[0].ask_sz, 2);
    ASSERT_EQ(msg->levels[0].bid_ct, 1);
    ASSERT_EQ(msg->levels[0].ask_ct, 3);
  }

  // Free the memory allocated by Rust
  destroy_record_decoder(decoder);
}

TEST(DecoderTests, test_decode_file) {
  // Prepare test records
  uint8_t length = sizeof(Mbp1Msg) / METADATA_LENGTH_MULTIPLIER;
  CRecordEnum record1{
      .rtype = RType::Mbp1,
      .data = RecordData{
          .mbp1 = {
              length, RType::Mbp1, 1,   1622471124, 0, 1000,
              10,     'T',         'A', 0,          0, 123456789098765,
              12345,  123456,      0,   1,          2, 2,
              2,      1,           3,
          }}};

  std::vector<RecordData> records;
  records.push_back(record1.data);

  // Create the CRecordEncoder
  CRecordEncoder* encoder = create_record_encoder();
  ASSERT_NE(encoder, nullptr) << "Failed to create CRecordEncoder";

  // Encode the records
  int result = encode_records(encoder, records.data(), records.size());
  EXPECT_EQ(result, 0) << "Failed to encode records";

  // Retrieve encoded data
  size_t encoded_size = 0;
  get_encoded_data(encoder, nullptr, &encoded_size);  // Query size first
  ASSERT_GT(encoded_size, 0) << "Encoded data size is zero";

  std::vector<uint8_t> encoded_data(encoded_size);
  get_encoded_data(encoder, encoded_data.data(), &encoded_size);

  // Write to file
  const char* file = "../tests/test_decode.bin";
  write_buffer_to_file(encoder, file, false);

  // Destroy the encoder
  destroy_record_encoder(encoder);

  // Test
  CRecordDecoder* decoder = create_file_decoder(file);

  CRecordEnum record;
  while (decoder_iter(decoder, &record)) {
    const Mbp1Msg* msg = &record.data.mbp1;
    ASSERT_EQ(msg->hd.rtype, RType::Mbp1);
    ASSERT_EQ(msg->hd.instrument_id, 1);
    ASSERT_EQ(msg->hd.rollover_flag, 0);
    ASSERT_EQ(msg->price, 1000);
    ASSERT_EQ(msg->size, 10);
    ASSERT_EQ(msg->action, Action::Trade);
    ASSERT_EQ(msg->side, Side::Ask);
    ASSERT_EQ(msg->depth, 0);
    ASSERT_EQ(msg->flags, 0);
    ASSERT_EQ(msg->ts_recv, 123456789098765);
    ASSERT_EQ(msg->ts_in_delta, 12345);
    ASSERT_EQ(msg->sequence, 123456);
    ASSERT_EQ(msg->discriminator, 0);
    ASSERT_EQ(msg->levels[0].bid_px, 1);
    ASSERT_EQ(msg->levels[0].ask_px, 2);
    ASSERT_EQ(msg->levels[0].bid_sz, 2);
    ASSERT_EQ(msg->levels[0].ask_sz, 2);
    ASSERT_EQ(msg->levels[0].bid_ct, 1);
    ASSERT_EQ(msg->levels[0].ask_ct, 3);
  }

  // Free the memory allocated by Rust
  destroy_record_decoder(decoder);
}
