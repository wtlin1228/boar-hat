# Define variables
CC = clang
CFLAGS = -I$(shell brew --prefix)/include -L$(shell brew --prefix)/lib
LIBS = -lglfw -framework OpenGL
SRC = ./src/main.cpp
OUT = ./build/main

# Default target
all: $(OUT)

# Build target
$(OUT): $(SRC)
	$(CC) -o $@ $< $(CFLAGS) $(LIBS)

# Clean target
clean:
	rm -f $(OUT)

# Phony targets
.PHONY: all clean
