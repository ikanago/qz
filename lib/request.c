#include "request.h"

#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <unistd.h>

#include "parse_request.h"

int http_session(const int sock) {
    char buf[256];
    Request req;
    const int size = recv(sock, buf, 256, 0);
    if (size <= 0) {
        return -1;
    }
    buf[size] = 0;
    const Result res = parse_status_line(buf, size + 1, &req);
    if (is_err(res)) {
        return -1;
    }
    check_file(&req, "html");
    reply_http(sock, &req);
    return 0;
}

void check_file(Request* req, const char* dir_name) {
    snprintf(req->real_path, sizeof(req->real_path), "%s%s", dir_name, req->uri);
    struct stat s;
    int ret = stat(req->real_path, &s);
    if (S_ISDIR(s.st_mode)) {
        sprintf(req->real_path, "%sindex.html", req->real_path);
        sprintf(req->uri, "%sindex.html", req->uri);
    }
    ret = stat(req->real_path, &s);
    if (ret == -1) {
        req->code = 404;
    } else {
        req->code = 200;
        req->size = (int)s.st_size;
    }
    printf("%s\n", req->real_path);

    const char* extension = strstr(req->real_path, ".");
    if (extension == NULL) {
        return;
    }
    if (strncmp(extension, ".html", 5) == 0) {
        strncpy(req->type, "text/html", 10);
    } else if (strncmp(extension, ".jpg", 4) == 0) {
        strncpy(req->type, "image/jpeg", 11);
    }
}

void reply_http(const int sock, Request* req) {
    if (req->code == 404) {
        send_404(sock);
        return;
    }

    char buf[256];
    size_t len = snprintf(buf, sizeof(buf), "HTTP/1.1 200 OK\r\nContent-Length: %d\r\nContent-Type: %s\r\n\r\n", req->size, req->type);

    const int res = send(sock, buf, len, 0);
    if (res < 0) {
        shutdown(sock, SHUT_RDWR);
        close(sock);
        return;
    }
    send_file(sock, req->real_path, req->size);
}

void send_404(const int sock) {
    char buf[256];
    snprintf(buf, sizeof(buf), "HTTP/1.1 404 Not Found\r\n\r\n");
    printf("%s\n", buf);
    const int res = send(sock, buf, strlen(buf), 0);
    if (res < 0) {
        shutdown(sock, SHUT_RDWR);
        close(sock);
    }
}

void send_file(const int sock, const char* file_name, const size_t size) {
    const int fd = open(file_name, O_RDONLY);
    if (fd == -1) {
        shutdown(sock, SHUT_RDWR);
        close(sock);
        return;
    }

    char* buf = calloc(size, sizeof(char));
    if (buf == NULL) {
        shutdown(sock, SHUT_RDWR);
        close(sock);
    }
    int len = read(fd, buf, size);
    const int ret = send(sock, buf, len, 0);
    if (ret < 0) {
        shutdown(sock, SHUT_RDWR);
        close(sock);
        free(buf);
    }
    free(buf);
    close(fd);
}
