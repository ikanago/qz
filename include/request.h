#ifndef REQUEST_H_
#define REQUEST_H_

#include <stddef.h>

typedef struct {
    char method[64];
    char uri[256];
    char real_path[256];
    char type[64];
    int code;
    int size;
} Request;

int http_session(const int sock);

void check_file(Request* req, const char* dir_name);

void reply_http(const int sock, Request* req);

void send_file(const int sock, const char* file_name, const size_t size);

void send_404(const int sock);

#endif
