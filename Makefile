CC := gcc
CXX := g++
FORMAT := clang-format
PYTHON := python

FORMAT_OPT := -style=file -i

SRCDIR := src
TESTDIR := test
INCLUDE := include
OBJDIR := obj
BINDIR := bin
GTEST_OUTPUT_DIR := gtest_output
GTEST_ROOT_DIR := thirdparty/googletest

SRC = $(wildcard $(SRCDIR)/*.c)
HEADERS = $(wildcard $(INCLUDE)/*.h)
OBJ = $(addprefix $(OBJDIR)/, $(notdir $(SRC:.c=.o)))
TESTSRC = $(wildcard $(TESTDIR)/*.cpp)
TESTOBJ = $(addprefix $(OBJDIR)/, $(notdir $(TESTSRC:.cpp=.o)))

CFLAGS += -I$(INCLUDE)
CPPFLAGS += -I$(GTEST_OUTPUT_DIR) -I$(INCLUDE) -DGTEST_HAS_PTHREAD=0

.PHONY: format
format:
	@$(FORMAT) $(FORMAT_OPT) $(SRC) $(HEADERS)

.PHONY: clean
clean:
	rm -rf $(OBJDIR) $(BINDIR) $(GTEST_OUTPUT_DIR)

.PHONY: test
test: $(BINDIR)/test_main
	@mkdir -p $(BINDIR)
	./$(BINDIR)/test_main

$(OBJ): $(SRC) $(HEADERS)
	@mkdir -p $(OBJDIR) $(BINDIR)
	$(CC) $(CFLAGS) -c $(SRC) -o $@

$(GTEST_OUTPUT_DIR)/gtest:
	$(PYTHON) $(GTEST_ROOT_DIR)/googletest/scripts/fuse_gtest_files.py $(GTEST_OUTPUT_DIR)

$(BINDIR)/test_main: $(OBJDIR)/gtest-all.o $(OBJDIR)/gtest_main.o $(TESTOBJ) $(OBJ)
	@echo $(TESTOBJ)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -o $@ $^

$(OBJDIR)/gtest-all.o: $(GTEST_OUTPUT_DIR)/gtest
	@mkdir -p $(OBJDIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $(OBJDIR)/gtest-all.o $(GTEST_OUTPUT_DIR)/gtest/gtest-all.cc

$(OBJDIR)/gtest_main.o: $(GTEST_OUTPUT_DIR)/gtest
	@mkdir -p $(OBJDIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $(OBJDIR)/gtest_main.o $(GTEST_ROOT_DIR)/googletest/src/gtest_main.cc

$(TESTOBJ): $(GTEST_OUTPUT_DIR)/gtest $(OBJ) $(HEADERS)
	@mkdir -p $(OBJDIR) $(BINDIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $(TESTOBJ) $(TESTSRC)

