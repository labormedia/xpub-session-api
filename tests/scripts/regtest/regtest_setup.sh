#!/bin/bash
echo 'Shutting down previous instances'
killall bitcoind
source ./tests/scripts/alias_source.sh
sleep 5
$btd
sleep 5
echo 'working dir '$(pwd)
echo 'Creating wallets'
$bt -named createwallet wallet_name=benefactor blank=true 
$bt -named createwallet wallet_name=beneficiary blank=true 
