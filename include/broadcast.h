#pragma once

#include <stdlib.h>
#include <sys/select.h>

void init_clients(int* clients, const size_t num_clients);

void set_fds(fd_set* pfds, const int sock, int* clients, const size_t num_clients);

void add_client(const int sock, int* clients, const size_t num_clients);

void remove_client(const int id, int* clients);

int get_max_sock(int* clients, const size_t num_clients);

void broadcast(char* buf, const size_t buf_len, const int socket_from, int* clients,
               const size_t num_clients);
