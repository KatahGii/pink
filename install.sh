#!/bin/bash

if [ ! -f "target/release/pink" ]; then
  echo "Error: The pink binary was not found. Please run 'cargo build --release' first."
  exit 1
fi

if [ ! -f "pink.pink" ]; then
  echo "Error: The pink.pink file was not found in the project directory."
  exit 1
fi

mkdir -p /usr/local/share/pink

cp pink.pink /usr/local/share/pink/

sudo mv target/release/pink /usr/local/bin/

echo "Installation successful! You can now run 'pink' from anywhere in the terminal."
