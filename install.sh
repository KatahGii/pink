#!/bin/sh

if [ ! -f "target/release/pink" ]; then
  echo "Error: The pink binary was not found. Please run 'cargo build --release' first."
  exit 1
fi

if [ ! -f "pink.pink" ]; then
  echo "Error: The pink.pink file was not found in the project directory."
  exit 1
fi

OS=$(uname -s)

if [ "$OS" = "Darwin" ]; then
  # macOS
  PINK_DIR="/usr/local/share/pink"
  BIN_DIR="/usr/local/bin"
elif [ "$OS" = "FreeBSD" ]; then
  # FreeBSD
  PINK_DIR="/usr/local/share/pink"
  BIN_DIR="/usr/local/bin"
else
  echo "Error: Unsupported operating system. This script is intended for macOS and FreeBSD."
  exit 1
fi

mkdir -p "$PINK_DIR"

cp pink.pink "$PINK_DIR/"

sudo mv target/release/pink "$BIN_DIR/"

echo "Installation successful! You can now run 'pink' from anywhere in the terminal."
