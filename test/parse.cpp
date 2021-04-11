#include <stdio.h>
#include <string.h>

#include <iostream>

#include "gmock/gmock.h"

extern "C" {
#include "parse.h"
}

TEST(ParseRequest, ValidRequestLine) {
    char line[] = "GET /index.html HTTP/1.1";
    Info info;
    ASSERT_TRUE(parse_status_line(line, strlen(line), &info));
    ASSERT_THAT("GET", testing::ElementsAreArray(info.method, 4));
    ASSERT_THAT("/index.html", testing::ElementsAreArray(info.uri, 12));
}
