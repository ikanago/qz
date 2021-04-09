CC = gcc
FORMAT = clang-format

FORMAT_OPT := -style=file -i

SRCS = $(wildcard *.c)
HEADERS = $(wildcard *.h)

.PHONY: format
format:
	@$(FORMAT) $(FORMAT_OPT) $(SRCS) $(HEADERS)

