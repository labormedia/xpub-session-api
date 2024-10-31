#!/bin/bash
killall bitcoind
bitcoind -regtest -daemon
sleep 5
echo 'working dir '$(pwd)
source ./tests/scripts/alias_source.sh
echo 'Creating wallets'
$bt -named createwallet wallet_name=benefactor blank=true
$bt -named createwallet wallet_name=beneficiary blank=true
