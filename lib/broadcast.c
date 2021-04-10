#include "broadcast.h"

#include <stdio.h>
#include <stdlib.h>
#include <sys/select.h>
#include <unistd.h>

void init_clients(int* clients, const size_t num_clients) {
    for (size_t i = 0; i < num_clients; i++) {
        clients[i] = 0;
    }
}

void set_fds(fd_set* pfds, const int sock, int* clients,
             const size_t num_clients) {
    FD_ZERO(pfds);
    FD_SET(sock, pfds);
    for (size_t i = 0; i < num_clients; i++) {
        if (clients[i] == 1) {
            FD_SET(i, pfds);
        }
    }
}

void add_client(const int sock, int* clients, const size_t num_clients) {
    if (sock < num_clients) {
        clients[sock] = 1;
    } else {
        fprintf(stderr, "Connection overflow\n");
        exit(1);
    }
}

void remove_client(const int id, int* clients) { clients[id] = 0; }

int get_max_sock(int* clients, size_t num_clients) {
    int max_sock = 0;
    for (int i = 0; i < num_clients; i++) {
        if (clients[i] == 1) {
            max_sock = i;
        }
    }
    return max_sock;
}

void broadcast(char* buf, const size_t buf_len, const int socket_from,
               int* clients, const size_t num_clients) {
    for (int i = 0; i < num_clients; i++) {
        if (i == socket_from) {
            continue;
        }

        if (clients[i] == 1) {
            write(i, buf, buf_len);
        }
    }
}
