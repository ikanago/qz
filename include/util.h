#ifndef UTIL_H_
#define UTIL_H_

#include <stddef.h>

/**
 * Returns the number of bytes which is read from `buf` until `target` appears.
 * e.g. read_until("hoge fuga", ' ') -> 4.
 */
size_t read_until(const char* buf, char target);

/**
 * Copy characters to `output` until `target` appears and appends '\0' to `output`.
 * Make sure that `output` has enough size to store the characters.
 * Returns the number of bytes which is read from `buf`, which equals to `strlen(output)`.
 * e.g. read_until("hoge fuga", ' ') -> 4.
 */
size_t consume_until(const char* buf, char* output, char target);

#endif
