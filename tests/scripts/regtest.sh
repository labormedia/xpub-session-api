#!/bin/bash
echo -e "Setting up for keys \n$1 \nand $2\n"
source ./tests/scripts/alias_source.sh
first_derivation="($1/86'/1'/0'/1/*)"
second_derivation="($2/86'/1'/0'/1/*)"
$bt_benefactor importdescriptors '[ 
    { "desc": "tr'"$first_derivation"'#82mnd0h5", "active": true, "timestamp": "now", "internal": true }, 
    { "desc": "tr'"$first_derivation"'#82mnd0h5", "active": true, "timestamp": "now" } 
]'
$bt_beneficiary importdescriptors '[ 
    { "desc": "tr'"$second_derivation"'#4ka5r09z", "active": true, "timestamp": "now", "internal": true }, 
    { "desc": "tr'"$second_derivation"'#4ka5r09z", "active": true, "timestamp": "now" } 
]' 
$bt generatetoaddress 103 $($bt_benefactor getnewaddress '' bech32m)