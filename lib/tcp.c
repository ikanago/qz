#include "tcp.h"

#include <netinet/in.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>
#include <sys/socket.h>
#include <unistd.h>

int tcp_listen(const int port) {
    const int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        perror("Socket: ");
        exit(1);
    }

    struct sockaddr_in addr;
    bzero(&addr, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    addr.sin_port = htons(port);
    const int yes = 1;
    setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(yes));

    const int bind_res = bind(sock, (struct sockaddr *)&addr, sizeof(addr));
    if (bind_res < 0) {
        close(sock);
        perror("Bind: ");
        exit(1);
    }

    const int listen_res = listen(sock, 5);
    if (listen_res < 0) {
        close(sock);
        perror("Listen: ");
        exit(1);
    }
    return sock;
}

int tcp_accept(const int sock_listen) {
    struct sockaddr addr;
    const size_t len = sizeof(struct sockaddr);
    return accept(sock_listen, &addr, (socklen_t *)&len);
}

int tcp_connect(const char* hostname, const int port) {
    const int sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    const struct hostent* host = gethostbyname(hostname);

    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr = *(struct in_addr *)(host->h_addr_list[0]);
    addr.sin_port = htons(port);

    int ret;
    if ((ret = connect(sock, (struct sockaddr*)&addr, sizeof(addr))) < 0) {
        close(sock);
        perror("Connect: ");
        exit(1);
    } else {
        return sock;
    }
}
