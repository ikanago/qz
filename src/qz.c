#include <fcntl.h>
#include <stdio.h>
#include <strings.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <unistd.h>

#include "tcp.h"

#define BUFSIZE 1024

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: %s [port]\n", argv[0]);
        return 1;
    }

    const int port = atoi(argv[1]);
    const int sock_listen = tcp_listen(port);
    char buf[BUFSIZE];
    while (1) {
        const int sock_accept = tcp_accept(sock_listen);
        int ret = 1;
        while ((ret = tcp_talk(sock_accept, buf, BUFSIZE)) == 1) {
        }
        close(sock_accept);
    }

    close(sock_listen);
    return 0;
}
