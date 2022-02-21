#!/bin/bash

# source neardev/dev-account.env
# near delete $CONTRACT_NAME

rm -rf neardev .env

near dev-deploy res/pitch_talk.wasm
source neardev/dev-account.env
cat neardev/dev-account.env > .env

echo "Initializing PitchTalk contract '$CONTRACT_NAME'"
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME

echo "Creating author1 account"
near create-account author1.$CONTRACT_NAME --masterAccount $CONTRACT_NAME --initialBalance "1"

echo "Creating investor1 account"
near create-account investor1.$CONTRACT_NAME --masterAccount $CONTRACT_NAME --initialBalance "1"

PITCH_TALK=$CONTRACT_NAME
AUTHOR=author1.$CONTRACT_NAME
INVESTOR=investor1.$CONTRACT_NAME

#copy dev contract to cli
\cp -r .env ../../contract-cli/

echo "PitchTalk='$PITCH_TALK'"
echo "Author='$AUTHOR'"
echo "Investor='$INVESTOR'"
