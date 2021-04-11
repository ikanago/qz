#pragma once

#include <stdlib.h>

/**
 * Returns the number of bytes which is read from `buf` until `target` appears.
 * e.g. read_until("hoge fuga", ' ') -> 4.
 */
size_t read_until(const char* buf, const char* buf_end,  char target);

/**
 * Copy characters to `output` until `target` appears.
 * Make sure that `output` has enough size to store the characters.
 * Returns the number of bytes which is read from `buf`.
 */
size_t consume_until(const char* buf, const char* buf_end, char* output, char target);

