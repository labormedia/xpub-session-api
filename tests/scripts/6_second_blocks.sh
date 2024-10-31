#!/bin/bash
source ./tests/scripts/alias_source.sh
while :
do
        echo "[REGTEST] Generate a new block `date '+%d/%m/%Y %H:%M:%S'`"
        $bt generatetoaddress 1 $($bt_benefactor getnewaddress '' bech32m)
        sleep 6
done