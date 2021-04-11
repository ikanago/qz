#include "qz.h"

#include <fcntl.h>
#include <stdio.h>
#include <strings.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <unistd.h>

#include "broadcast.h"
#include "tcp.h"
#include "util.h"

extern void serve_broadcast(const int sock_listen, int* clients);

int main(int argc, char** argv) {
    /*
    if (argc != 2) {
        printf("Usage: %s [port]\n", argv[0]);
        return 1;
    }

    const int port = atoi(argv[1]);
    const int sock_listen = tcp_listen(port);
    int clients[NUM_CLIENTS];
    init_clients(clients, NUM_CLIENTS);
    while (1) {
        serve_broadcast(sock_listen, clients);
    }

    close(sock_listen);
    */
    char line[] = "hoge fuga";
    char* buf = &line[0];
    char output[8];
    consume_until(buf, buf + 9, output, ' ');
    printf("%c\n", *buf);
    return 0;
}
