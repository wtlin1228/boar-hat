# Minimum CMake version. 3.20 gives us modern features and reasonable error messages.
cmake_minimum_required(VERSION 3.20)

# Name the project and tell CMake we only use C++ (not C, Fortran, etc.).
# Skipping LANGUAGES makes CMake also probe for a C compiler, which we don't need.
project(learncpp LANGUAGES CXX)

# Use the C++20 standard for every target in this project.
set(CMAKE_CXX_STANDARD 20)

# Fail the build if the compiler can't actually provide C++20
# (without this, CMake silently falls back to an older standard).
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Disable compiler-specific extensions (e.g. GNU extensions like __typeof__).
# Keeps the code portable / standards-conformant.
set(CMAKE_CXX_EXTENSIONS OFF)

# Generate build/compile_commands.json — a list of every compile command.
# clangd reads this to know which flags / include paths your code uses,
# which is what powers accurate intellisense and diagnostics in VSCode.
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Default to a Debug build (no optimizations, debug symbols on) when the
# user didn't specify. For learning C++ this is what you almost always want.
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

# Turn on a generous set of compiler warnings for every target.
#   -Wall         common warnings
#   -Wextra       extra warnings -Wall misses
#   -Wpedantic    warn on non-standard C++ usage
#   -Weffc++      Scott Meyers' "Effective C++" style guidelines
#                 (e.g. classes with pointer members need explicit copy ctors).
#                 Noisy on real codebases; great for learning idiomatic C++.
#   -Wconversion       warn on implicit type conversions that may change a value
#                      (e.g. int -> float, double -> int).
#                      Catches a whole class of subtle numeric bugs.
#   -Wsign-conversion  warn specifically on signed <-> unsigned conversions.
#                      (Implied by -Wconversion in C++, but listed for clarity.)
#   -Werror       promote ALL warnings to errors — the build fails until you fix them.
#                 Strict, but forces good habits while learning.
# Catching mistakes early is one of the best things you can do as a beginner.
add_compile_options(
    -Wall
    -Wextra
    -Wpedantic
    -Weffc++
    -Wconversion
    -Wsign-conversion
    -Werror
)

# Auto-discover every exercise: list all entries inside projects/ and,
# for each one that's a directory containing its own CMakeLists.txt, include it.
# This means adding a new exercise = create projects/<Name>/CMakeLists.txt;
# no edits to this file required.
file(GLOB project_dirs RELATIVE ${CMAKE_CURRENT_SOURCE_DIR}/projects
    ${CMAKE_CURRENT_SOURCE_DIR}/projects/*)
foreach(dir ${project_dirs})
    if(IS_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/projects/${dir}
       AND EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/projects/${dir}/CMakeLists.txt)
        add_subdirectory(projects/${dir})
    endif()
endforeach()
