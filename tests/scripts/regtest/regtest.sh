#!/bin/bash
echo -e "Syntax: regtest.sh [XPRIV1] [XPRIV2]"
source ./tests/scripts/alias_source.sh
first_derivation="($1/86'/1'/0'/1/*)#rtyydy5a" 
second_derivation="($2/86'/1'/0'/1/*)#ezynu034"
$bt_benefactor importdescriptors '[ 
    { "desc": "tr'"$first_derivation"'", "active": true, "timestamp": "now", "internal": true }, 
    { "desc": "tr'"$first_derivation"'", "active": true, "timestamp": "now" } 
]'
$bt_beneficiary importdescriptors '[ 
    { "desc": "tr'"$second_derivation"'", "active": true, "timestamp": "now", "internal": true }, 
    { "desc": "tr'"$second_derivation"'", "active": true, "timestamp": "now" } 
]' 
#$bt generatetoaddress 103 $($bt_benefactor getnewaddress '' bech32m)