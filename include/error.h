#ifndef ERROR_H_
#define ERROR_H_

typedef enum {
    Ok,

    // Error while parse request
    MethodNotAllowed,
    InvalidUri,
    InvalidVersion,
} Result;

int is_ok(const Result res);

int is_err(const Result res);

#endif
