btd='bitcoind -regtest -server -daemon -rpcport=18443 -rpcuser=project -rpcpassword=xpub_manager -fallbackfee=0.0002 -rpcallowip=127.0.0.1/0 -rpcbind=127.0.0.1 -blockfilterindex=1 -peerblockfilters=1'
bt='bitcoin-cli -regtest -rpcuser=project -rpcpassword=xpub_manager'
bt_benefactor='bitcoin-cli -regtest -rpcwallet=benefactor -rpcuser=project -rpcpassword=xpub_manager'
bt_beneficiary='bitcoin-cli -regtest -rpcwallet=beneficiary -rpcuser=project -rpcpassword=xpub_manager'