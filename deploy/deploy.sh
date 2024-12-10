#!/bin/bash

export SEED="[your seed phrase]"
export URL="wss://ws.test.azero.dev"
export CONTRACT_PATH="./target/ink/mytoken.contract"

echo "Deploying contract..."
cargo contract instantiate \
    --suri "$SEED" \
    --url "$URL" \
    --constructor new \
    --args 1000
