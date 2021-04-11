#ifndef TCP_H_
#define TCP_H_

#include <stddef.h>

int tcp_listen(const int port);

int tcp_accept(const int sock_listen);

int tcp_connect(const char* hostname, const int port);

int tcp_talk(const int sock, char* buf, const size_t buf_len);

#endif
