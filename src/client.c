#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>
#include <sys/socket.h>
#include <unistd.h>

#include "qz.h"
#include "tcp.h"

int main(int argc, char** argv) {
    if (argc != 3) {
        printf("Usage: %s [ip address] [port]\n", argv[0]);
        return 1;
    }

    const int port = atoi(argv[2]);
    const int sock = tcp_connect(argv[1], port);
    char buf[BUFSIZE];
    int ret = 1;
    while ((ret = tcp_talk(sock, buf, BUFSIZE)) == 1) {
    }

    close(sock);
    return 0;
}
