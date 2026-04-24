#!/bin/bash
set -euo pipefail

echo "Building Ọ̀ṣỌ́ core → WebAssembly..."

cd "$(dirname "$0")/../core/parser"

# Generate JS/TS bindings with wasm-pack
if command -v wasm-pack &> /dev/null; then
    wasm-pack build --target web --out-dir ../wasm/pkg --features wasm
    echo "WASM build complete → core/wasm/pkg/"
    echo ""
    echo "To use in frontend:"
    echo "  import init, { parse, validate } from '@/core/wasm/pkg/oso_parser'"
else
    echo "wasm-pack not found. Install with:"
    echo "  cargo install wasm-pack"
    echo ""
    echo "Falling back to raw cargo build..."
    cargo build --target wasm32-unknown-unknown --release --features wasm
    echo "Raw WASM at: target/wasm32-unknown-unknown/release/oso_parser.wasm"
    exit 1
fi
