#include "error.h"

int is_ok(const Result res) {
    return res == Ok;
}

int is_err(const Result res) {
    return res != Ok;
}
