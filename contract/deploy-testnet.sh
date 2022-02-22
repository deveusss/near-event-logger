#!/bin/bash

# source neardev/dev-account.env
# near delete $CONTRACT_NAME

rm -rf neardev .env

near dev-deploy res/contract.wasm
source neardev/dev-account.env
cat neardev/dev-account.env > .env

echo "Initializing EventLogger contract '$CONTRACT_NAME'"
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME

echo "EventLogger='$CONTRACT_NAME'"

