#!/bin/bash

export SEED="[your seed phrase]"
export URL="wss://ws.test.azero.dev"
export CONTRACT="[deployed contract address]"

echo "Checking total supply..."
cargo contract call \
    --suri "$SEED" \
    --url "$URL" \
    --contract "$CONTRACT" \
    --message total_supply \
    --dry-run

echo "Transferring tokens..."
cargo contract call \
    --suri "$SEED" \
    --url "$URL" \
    --contract "$CONTRACT" \
    --message transfer \
    --args [recipient address] 100
