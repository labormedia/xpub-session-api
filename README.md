# xpub-session-api

This project has a strong focus on the security property of non-custodial addresses. This derives in a business logic that doesn't consider the manipulation of private keys directly, rather the verification and authentication via cryptographic signatures. In the case of multi-party management of funds, multi-signature schemes are provided by the Bitcoin standards considered for this implementation.

## Run

Initialize storage:

```console
docker run -p 27017:27017 --name mongodb -d mongodb/mongodb-community-server:latest
docker run -p 6379:6379 --name actix-redis -d redis
```

```console
cargo run --release
```

## Test

Requirement: Bitcoin Core (https://bitcoin.org/en/bitcoin-core/)

Integration test scripts relies on bitcoin-cli and bitcoind applications from the Bitcoin Core project. They should be available in the path for script (/tests/scripts).

Regtest setup is needed only one time per terminal session. You could consider to delete the bitcoin core temporary regtest folder for instancing a clean test (caution is required for the deleting process, make sure not to delete relevant files and mainnet):
```console
./tests/scripts/regtest_setup.sh
```

For test context execution:
```console
./test/scripts/regtest.sh [XPRIV_KEY_1] [XPRIV_KEY_2]
```

Xpriv key generation is provided on screen in the example "generate_xpriv":

```console
cargo run --release --example --example generate_xpriv
```