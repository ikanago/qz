#pragma once

int tcp_listen(const int port);

int tcp_accept(const int sock_listen);

int tcp_connect(const char* hostname, const int port);
