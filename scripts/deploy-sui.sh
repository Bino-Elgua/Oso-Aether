#!/bin/bash
set -euo pipefail

echo "Deploying Ọ̀ṣỌ́ contracts to Sui..."

NETWORK="${1:-testnet}"

cd "$(dirname "$0")/../contracts"

echo "Target network: $NETWORK"
echo "Publishing package..."

sui client publish --gas-budget 100000000 --skip-dependency-verification

echo "Contracts deployed."
