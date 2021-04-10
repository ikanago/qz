#include <stdio.h>
#include <strings.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <unistd.h>

#include "broadcast.h"
#include "qz.h"
#include "tcp.h"

void serve_broadcast(const int sock_listen, int* clients) {
    fd_set fds;
    set_fds(&fds, sock_listen, clients, NUM_CLIENTS);
    int max_sock = get_max_sock(clients, NUM_CLIENTS);
    if (max_sock < sock_listen) {
        max_sock = sock_listen;
    }

    select(max_sock + 1, &fds, NULL, NULL, NULL);

    if (FD_ISSET(sock_listen, &fds) != 0) {
        const int sock = tcp_accept(sock_listen);
        add_client(sock, clients, NUM_CLIENTS);
        printf("Joined new client %d\n", sock);
    }

    char buf[1024];
    for (int i = 0; i < NUM_CLIENTS; i++) {
        if (clients[i] == 0) {
            continue;
        }

        if (FD_ISSET(i, &fds) != 0) {
            const int received_bytes = recv(i, buf, BUFSIZE, 0);
            if (received_bytes > 0) {
                write(STDOUT_FILENO, buf, received_bytes);
                broadcast(buf, received_bytes, i, clients, NUM_CLIENTS);
            } else {
                remove_client(i, clients);
                printf("Removed client %d\n", i);
            }
        }
    }
}
