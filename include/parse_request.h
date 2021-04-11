#ifndef PARSE_REQUEST_H_
#define PARSE_REQUEST_H_

#include <stddef.h>

#include "error.h"

typedef struct {
    char method[64];
    char uri[256];
    char real_path[256];
    char type[64];
    int code;
    int size;
} Request;

/**
 * Parse `line` as an HTTP request and extract information about request header.
 */
Result parse_status_line(const char* line, const size_t size, Request* info);

/**
 * Check if `method` represents valid HTTP method.
 */
Result validate_method(const char* method, const size_t method_len);

/**
 * Check if `uri` represents valid URI.
 */
Result validate_uri(const char* uri, const size_t uri_len);

/**
 * Check if `version` represents valid URI.
 */
Result validate_version(const char* version, const size_t version_len);

#endif
