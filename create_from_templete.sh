#!/bin/sh


if [ $# -lt 2 ]; then
  echo "need new directory name and package_name"
  echo "eg.  $0 2025 01"
  exit 1;
fi

# always work from the repo root (where this script lives), regardless of caller's cwd
cd "$(dirname "$0")" || exit 1

name="$1/$1-$2"
package="aoc_$1_$2"

if [ -d "./$name" ]; then
    echo "exist directory, use another name"
    exit 1;
fi

mkdir -p "$1"
cp -r _template "$name"
find "$name" -type f -exec sed -i 's@package_name@'"$package"'@g' {} +
./refresh.sh

# a script can't change its parent shell's directory, so open a new shell inside the created directory
cd "$name" || exit 1
echo "entering $(pwd)  (type 'exit' to return)"
exec "${SHELL:-/bin/sh}"
