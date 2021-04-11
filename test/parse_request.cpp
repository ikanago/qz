#include <stdio.h>
#include <string.h>

#include <iostream>

#include "gmock/gmock.h"

extern "C" {
#include "error.h"
#include "parse_request.h"
}

TEST(ParseRequest, ValidRequestLine) {
    char line[] = "GET /index.html HTTP/1.1";
    Request req;
    const Result r = parse_status_line(line, strlen(line), &req);
    ASSERT_TRUE(is_ok(r));
    ASSERT_THAT("GET", testing::ElementsAreArray(req.method, 4));
    ASSERT_THAT("/index.html", testing::ElementsAreArray(req.uri, 12));
}

TEST(ValidateMethod, ValidMethod) {
    char method[] = "GET";
    const Result r = validate_method(method, strlen(method));
    ASSERT_TRUE(is_ok(r));
}

TEST(ValidateUri, ValidUri) {
    char uri[] = "/index.html";
    const Result r = validate_uri(uri, strlen(uri));
    ASSERT_TRUE(is_ok(r));
}

TEST(ValidateUri, InvalidUri) {
    char uri[] = "index.html";
    const Result r = validate_uri(uri, strlen(uri));
    ASSERT_TRUE(is_err(r));
}

TEST(ValidateVersion, ValidVersion) {
    char version[] = "HTTP/1.1";
    const Result r = validate_version(version, strlen(version));
    ASSERT_TRUE(is_ok(r));
}

TEST(ValidateVersion, IsvalidVersion) {
    char version[] = "HTTP/2";
    const Result r = validate_version(version, strlen(version));
    ASSERT_TRUE(is_err(r));
}
