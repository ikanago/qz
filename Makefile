# Commands
CC := gcc
CXX := g++
FORMAT := clang-format
PYTHON := python

# Command options
FORMAT_OPT := -style=file -i
CFLAGS += -Wall -I$(INCLUDE)
CPPFLAGS += -I$(GTEST_OUTPUT_DIR) -I$(INCLUDE) -DGTEST_HAS_PTHREAD=0
CXXFLAGS += -Wall

# Directories
SRC_DIR := src
TEST_DIR := test
INCLUDE := include
OBJ_DIR := obj
TEST_OBJ_DIR := obj/test
BIN_DIR := bin
GTEST_OUTPUT_DIR := gtest_output
GTEST_ROOT_DIR := thirdparty/googletest

# Files
SRC = $(wildcard $(SRC_DIR)/*.c)
HEADERS = $(wildcard $(INCLUDE)/*.h)
OBJ = $(addprefix $(OBJ_DIR)/, $(notdir $(SRC:.c=.o)))
TEST_SRC = $(wildcard $(TEST_DIR)/*.cpp)
TEST_OBJ = $(addprefix $(TEST_OBJ_DIR)/, $(notdir $(TEST_SRC:.cpp=.o)))
GTEST_ALL_OBJ = $(OBJ_DIR)/gtest-all.o
GTEST_MAIN_OBJ = $(OBJ_DIR)/gtest_main.o
TARGET = qz

.PHONY: all
all: $(TARGET)

.PHONY: run
run: $(TARGET)
	@./$(TARGET)

.PHONY: fmt
fmt:
	@$(FORMAT) $(FORMAT_OPT) $(SRC) $(HEADERS)

.PHONY: clean
clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR) $(GTEST_OUTPUT_DIR)

.PHONY: test
test: $(BIN_DIR)/test_main
	@mkdir -p $(BIN_DIR)
	./$(BIN_DIR)/test_main

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c -o $@ $<

$(GTEST_OUTPUT_DIR)/gtest:
	$(PYTHON) $(GTEST_ROOT_DIR)/googletest/scripts/fuse_gtest_files.py $(GTEST_OUTPUT_DIR)

$(BIN_DIR)/test_main: $(GTEST_ALL_OBJ) $(GTEST_MAIN_OBJ) $(TEST_OBJ) $(OBJ)
	@mkdir -p $(dir $@)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -o $@ $(filter-out $(OBJ_DIR)/main.o, $^)

$(GTEST_ALL_OBJ): $(GTEST_OUTPUT_DIR)/gtest
	@mkdir -p $(dir $@)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $(OBJ_DIR)/gtest-all.o $(GTEST_OUTPUT_DIR)/gtest/gtest-all.cc

$(GTEST_MAIN_OBJ): $(GTEST_OUTPUT_DIR)/gtest
	@mkdir -p $(dir $@)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $(OBJ_DIR)/gtest_main.o $(GTEST_ROOT_DIR)/googletest/src/gtest_main.cc

$(TEST_OBJ_DIR)/%.o: $(TEST_DIR)/%.cpp $(HEADERS) $(GTEST_OUTPUT_DIR)/gtest
	@mkdir -p $(dir $@)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $@ $<

