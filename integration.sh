#!/bin/bash

set -e

KEY="admin"
KEY_ADDR="kujira16g2rahf5846rxzp3fwlswy08fz8ccuwkgthh5j"
GAS="--gas-prices 0.25ukuji --gas auto --gas-adjustment 1.5 -y -b block --output json"

# if CODE_ID is not set
if [ -z "$CODE_ID" ]; then
CONTRACT=$(kujirad tx wasm store artifacts/kujira_denom_wrapper.wasm --from $KEY $GAS --keyring-backend test | jq -r '.txhash') && echo $CONTRACT
sleep 1
CODE_ID=$(kujirad query tx $CONTRACT --output json | jq -r '.logs[0].events[-1].attributes[1].value') && echo "Code ID: $CODE_ID"
sleep 1
fi
TOKEN=token-${CODE_ID}
JSON=\''{"admin": "'"$KEY_ADDR"'", "nonce": "'"$TOKEN"'"}'\'
NONCE="token-${CODE_ID}"
JSON='{"admin": "'"$KEY_ADDR"'", "nonce": "'"$NONCE"'"}'
if [ -z "$ADDR" ]; then
# Instantiate contract
TX_INIT=$(kujirad tx wasm instantiate $CODE_ID "$JSON" --label kujira-denom-wrapper --amount 100000000ukuji --admin $KEY_ADDR $GAS --from $KEY --keyring-backend test | jq -r '.txhash') && echo $TX_INIT
sleep 1
ADDR=$(kujirad query tx $TX_INIT --output json | jq -r '.logs[0].events[0].attributes[0].value') && echo "Contract Address: $ADDR"
sleep 1
fi
DENOM="factory/$ADDR/token-$CODE_ID"
echo "Denom Created: $DENOM"
# Mint 12M Tokens to Self
TX_MINT=$(kujirad tx wasm execute $ADDR '{"kujira_denom_msg":{"mint": {"recipient": "'"$KEY_ADDR"'", "amount": "1200000000", "denom": "'"$DENOM"'"}}}' $GAS --from $KEY --keyring-backend test | jq -r '.txhash') && echo $TX_MINT
# Query Balance of Self
BALANCE=$(kujirad query bank balances $KEY_ADDR --denom $DENOM --output json | jq -r '.amount') && echo "Balance: $BALANCE"
