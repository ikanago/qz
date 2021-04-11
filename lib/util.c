#include "util.h"

#include <string.h>
#include <stddef.h>

size_t read_until(const char* buf, char target) {
    const char* found = strchr(buf, (int)target);
    if (found == NULL) {
        return strlen(buf);
    }
    return found - buf;
}

size_t consume_until(const char* buf, char* output, char target) {
    const size_t len = read_until(buf, target);
    strncpy(output, buf, len);
    output[len] = 0;
    return len;
}
