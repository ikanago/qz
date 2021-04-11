#include <string.h>

#include "gmock/gmock.h"

extern "C" {
#include "util.h"
}

TEST(Util, ReadUntilWhitespace) {
    char line[] = "GET /index.html HTTP/1.1";
    const size_t len = read_until(line, ' ');
    ASSERT_EQ(3ul, len);
}

TEST(Util, ReadUntilEOF) {
    char line[] = "abcde";
    const size_t len = read_until(line, ' ');
    ASSERT_EQ(5ul, len);
}

TEST(Util, ConsumeUntilWhitespace) {
    char line[] = "GET /index.html HTTP/1.1";
    char output[8];
    memset(output, 0, 8);
    const size_t len = consume_until(line, output, ' ');
    ASSERT_EQ(3ul, len);
    ASSERT_THAT("GET", testing::ElementsAreArray(output, len + 1));
}

TEST(Util, ConsumeUntilEOF) {
    char line[] = "abcde";
    char output[8];
    memset(output, 0, 8);
    const size_t len = consume_until(line, output, ' ');
    ASSERT_EQ(5ul, len);
    ASSERT_THAT("abcde", testing::ElementsAreArray(output, len + 1));
}
