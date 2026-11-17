#!/bin/sh


if [ $# -lt 2 ]; then
  echo "need new directory name and package_name"
  echo "eg.  $0 2025 01"
  exit 1;
fi

name="$1/$1-$2"
package="aoc_$1_$2"

if [ -d ./$name ]; then
    echo "exist directory, use another name"
    exit 1;
fi

pwd=$(pwd)
mkdir -p $1
cp -r _template $name
find $name -type f -exec sed -i 's@package_name@'$package'@g' {} +
./refresh.sh