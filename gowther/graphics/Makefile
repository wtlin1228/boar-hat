CXX := clang++
CXXFLAGS := -std=c++20 -I$(shell brew --prefix)/include -L$(shell brew --prefix)/lib
LIBS = -lglfw -framework OpenGL
LIB_DIR := lib
SRC_DIR := src
DIST_DIR := dist

PROJECTS := $(shell find $(SRC_DIR) -type f -name "main.cpp" -exec dirname {} \;)
TARGETS := $(patsubst $(SRC_DIR)/%, $(DIST_DIR)/%/main, $(PROJECTS))

all: $(TARGETS)

$(DIST_DIR)/%/main: $(SRC_DIR)/%/* $(LIB_DIR)/*
	@echo "Building $@..."
	@mkdir -p $(DIST_DIR)/$*
	@cp $(wildcard $(SRC_DIR)/$*/*.vs) $(DIST_DIR)/$* 2>/dev/null || true
	@cp $(wildcard $(SRC_DIR)/$*/*.fs) $(DIST_DIR)/$* 2>/dev/null || true
	$(CXX) -o $@ $(CXXFLAGS) $(LIBS) -I./$(LIB_DIR) $(wildcard $(SRC_DIR)/$*/*.cpp) $(wildcard $(LIB_DIR)/*.cpp)
	@echo ""

echo:
	@echo $(PROJECTS)
	@echo $(TARGETS)

clean:
	@echo "Cleaning up..."
	rm -rf $(DIST_DIR)