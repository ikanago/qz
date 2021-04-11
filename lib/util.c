#include "util.h"

#include <string.h>

size_t read_until(const char* buf, const char* buf_end, char target) {
    size_t pos = 0;
    while (buf + pos < buf_end) {
        if (buf[pos] == target) {
            break;
        }
        pos++;
    }
    return pos;
}

size_t consume_until(const char* buf, const char* buf_end, char* output, char target) {
    const size_t len = read_until(buf, buf_end, target);
    strncpy(output, buf, len);
    output[len] = 0;
    return len;
}
