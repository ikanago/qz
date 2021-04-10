#include <fcntl.h>
#include <stdio.h>
#include <strings.h>
#include <sys/socket.h>
#include <unistd.h>

#include "tcp.h"

#define BUFSIZE 1024

int main(int argc, char** argv) {
    if (argc != 3) {
        printf("Usage: %s [ip address] [input filename]\n", argv[0]);
        return 1;
    }

    const int sock = tcp_connect(argv[1], 11111);
    const int rfd = open(argv[2], O_RDONLY);
    char buf[BUFSIZE];
    bzero(&buf, BUFSIZE);
    int read_byte;
    while ((read_byte = read(rfd, buf, BUFSIZE)) > 0) {
        send(sock, buf, BUFSIZE, 0);
    }
    puts("Sent bytes");

    close(rfd);
    close(sock);
    return 0;
}
