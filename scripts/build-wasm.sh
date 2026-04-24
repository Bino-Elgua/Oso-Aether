#!/bin/bash
set -euo pipefail

echo "Building Ọ̀ṣỌ́ core → WebAssembly..."

cd "$(dirname "$0")/../core/parser"

# Build for WASM target
cargo build --target wasm32-unknown-unknown --release

# Generate JS/TS bindings with wasm-pack
if command -v wasm-pack &> /dev/null; then
    wasm-pack build --target web --out-dir ../wasm/pkg
    echo "WASM build complete → core/wasm/pkg/"
else
    echo "wasm-pack not found. Install: cargo install wasm-pack"
    echo "Raw WASM at: target/wasm32-unknown-unknown/release/"
    exit 1
fi
