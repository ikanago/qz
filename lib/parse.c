#include "parse.h"

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "util.h"

int parse_status_line(char* line, size_t size, Info* info) {
    size_t pos = 0;
    const size_t method_len = consume_until(line + pos, line + size, info->method, ' ');
    pos += method_len + 1;
    const size_t uri_len = consume_until(line + pos, line + size, info->uri, ' ');
    pos += uri_len + 1;
    const size_t version_len = read_until(line + pos, line + size, '\r');
    pos += version_len + 1;
    return 1;
}
