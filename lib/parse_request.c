#include "parse_request.h"

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "error.h"
#include "method.h"
#include "request.h"
#include "util.h"

Result parse_status_line(const char* line, const size_t size, Request* req) {
    size_t pos = 0;
    const size_t method_len = consume_until(line + pos, line + size, req->method, ' ');
    const Result is_valid_method = validate_method(req->method, method_len);
    if (is_err(is_valid_method)) {
        return is_valid_method;
    }
    pos += method_len + 1;

    const size_t uri_len = consume_until(line + pos, line + size, req->uri, ' ');
    const Result is_valid_uri = validate_uri(req->uri, uri_len);
    if (is_err(is_valid_uri)) {
        return is_valid_uri;
    }
    pos += uri_len + 1;

    const size_t version_len = read_until(line + pos, line + size, '\r');
    const Result is_valid_version = validate_version(line + pos, version_len);
    if (is_err(is_valid_version)) {
        return is_valid_version;
    }
    pos += version_len + 1;
    return Ok;
}

Result validate_method(const char* method, const size_t method_len) {
    if (strncmp(method, HTTP_GET, method_len) == 0) {
        return Ok;
    }
    return MethodNotAllowed;
}

Result validate_uri(const char* uri, const size_t uri_len) {
    if (uri_len < 1 || uri[0] != '/') {
        return InvalidUri;
    }
    return Ok;
}

Result validate_version(const char* version, const size_t version_len) {
    if (strncmp(version, "HTTP/1.1", version_len) == 0) {
        return Ok;
    }
    return InvalidVersion;
}
