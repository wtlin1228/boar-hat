# Define variables
CC = g++
CFLAGS = -I$(shell brew --prefix)/include -L$(shell brew --prefix)/lib
LIBS = -lglfw -framework OpenGL
SRC_DIR = ./src
DIST_DIR = ./dist
SOURCES = $(shell find $(SRC_DIR) -name '*.cpp')
TARGETS = $(patsubst $(SRC_DIR)/%.cpp, $(DIST_DIR)/%, $(SOURCES))

# Default target: Build all binaries
all: $(TARGETS)

# Build each binary
$(DIST_DIR)/%: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CC) -o $@ $< $(CFLAGS) $(LIBS)

# Clean target
clean:
	rm -rf $(DIST_DIR)

# Phony targets
.PHONY: all clean
