#include <fcntl.h>
#include <stdio.h>
#include <strings.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <unistd.h>

#include "broadcast.h"
#include "request.h"
#include "tcp.h"
#include "util.h"

#define BUFSIZE 1024
#define NUM_CLIENTS 10

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: %s [port]\n", argv[0]);
        return 1;
    }

    const int port = atoi(argv[1]);
    const int sock_listen = tcp_listen(port);
    printf("Listening on port %d\n", port);
    while (1) {
        const int sock_client = tcp_accept(sock_listen);
        http_session(sock_client);
        shutdown(sock_client, SHUT_RDWR);
        close(sock_client);
    }

    close(sock_listen);
    return 0;
}
