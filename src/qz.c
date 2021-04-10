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
        printf("Usage: %s [output filename]\n", argv[0]);
        return -1;
    }

    const int sock_listen = tcp_listen(11111);
    const int sock_accept = tcp_accept(sock_listen);

    const int wfd = open(argv[1], O_WRONLY | O_CREAT | O_TRUNC, S_IRWXU);
    char buf[BUFSIZE];
    bzero(&buf, BUFSIZE);
    int received_bytes;
    while ((received_bytes = recv(sock_accept, buf, BUFSIZE, 0)) > 0) {
        write(wfd, buf, received_bytes);
    }
    puts("Received bytes\n");

    close(sock_listen);
    close(sock_accept);
    close(wfd);
    return 0;
}
