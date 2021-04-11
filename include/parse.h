#pragma once

#include <stdlib.h>

typedef struct {
    char method[64];
    char uri[256];
    char real_path[256];
    char type[64];
    int code;
    int size;
} Info;

int parse_status_line(char* line, size_t size, Info* info);
