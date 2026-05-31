#!/usr/bin/env bash
# Scaffold a new learncpp.com exercise: creates projects/<Name>/ with a one-line
# CMakeLists.txt and a starter main.cpp, then re-runs CMake configure so the new
# target is picked up. See README.md > "Creating a new project".
#
# Usage: ./new-project.sh <ProjectName>
#   e.g. ./new-project.sh Chapter2_Variables

set -euo pipefail

# Resolve the repo root from this script's location, so it works from any cwd.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

name="${1:-}"
if [[ -z "$name" ]]; then
  echo "Usage: ./new-project.sh <ProjectName>" >&2
  echo "  e.g. ./new-project.sh Chapter2_Variables" >&2
  exit 1
fi

# CMake target names (and our folder names) can't contain spaces or slashes.
if [[ "$name" =~ [[:space:]/] ]]; then
  echo "Error: project name '$name' must not contain spaces or slashes." >&2
  exit 1
fi

project_dir="$repo_root/projects/$name"
if [[ -e "$project_dir" ]]; then
  echo "Error: projects/$name already exists." >&2
  exit 1
fi

mkdir -p "$project_dir"

# One-line CMakeLists.txt: the first arg is the target (executable) name.
cat > "$project_dir/CMakeLists.txt" <<EOF
add_executable($name main.cpp)
EOF

# Minimal starter program.
cat > "$project_dir/main.cpp" <<'EOF'
#include <iostream>

int main() {
  std::cout << "Hello from a new project!\n";

  return 0;
}
EOF

# Re-run configure so CMake discovers the new folder.
cmake -S "$repo_root" -B "$repo_root/build" > /dev/null

echo "Created projects/$name/"
echo "  - CMakeLists.txt"
echo "  - main.cpp"
echo
echo "Build & run:"
echo "  cmake --build build && ./build/projects/$name/$name"
