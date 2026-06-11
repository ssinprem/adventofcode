#!/bin/bash
echo -e "[workspace]\nresolver = \"2\"\nmembers = [" > Cargo.toml
for deep in 20*; do
    echo "    \"${deep%/}\"," >> Cargo.toml
done;
echo "]" >> Cargo.toml

