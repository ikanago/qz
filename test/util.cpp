#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include <string.h>

extern "C" {
#include "util.h"
}

TEST(Util, ReadUntilWhitespace) {
    char line[] = "GET /index.html HTTP/1.1";
    char* line_end = line + strlen(line);
    const size_t len = read_until(line, line_end, ' ');
    ASSERT_EQ(3ul, len);
}

TEST(Util, ReadUntilEOF) {
    char line[] = "abcde";
    char* line_end = line + strlen(line);
    const size_t len = read_until(line, line_end, ' ');
    ASSERT_EQ(5ul, len);
}

TEST(Util, ConsumeUntilWhitespace) {
    char line[] = "GET /index.html HTTP/1.1";
    char* line_end = line + strlen(line);
    char output[8];
    memset(output, 0, 8);
    const size_t len = consume_until(line, line_end, output, ' ');

    ASSERT_EQ(3ul, len);
    ASSERT_THAT("GET", testing::ElementsAreArray(output, len + 1));
}

TEST(Util, ConsumeUntilEOF) {
    char line[] = "abcde";
    char* line_end = line + strlen(line);
    char output[8];
    memset(output, 0, 8);
    const size_t len = consume_until(line, line_end, output, ' ');

    ASSERT_EQ(5ul, len);
    ASSERT_THAT("abcde", testing::ElementsAreArray(output, len + 1));
}

