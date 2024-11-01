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

To create a valid Xpub address with it signed witness:

```console
cargo run --release --example sign_nonce -- [nonce]
```

For example:
```console
$ cargo run --release --example sign_nonce -- 0
    Finished `release` profile [optimized] target(s) in 0.19s
     Running `target/release/examples/sign_nonce 0`
nonce: "0"
Xpub Xpub { network: Test, depth: 2, parent_fingerprint: d1e0e33b, child_number: Normal { index: 0 }, public_key: PublicKey(545c8ef02941861ee193abbaaa01e6a8b4806b7f5c99be31da018b209375d44de81f9b026810eac16e6fdcaa0bb13b068ecce3c338adb3a9190a2a171e5a1c9b), chain_code: f72adddcd35b4b193f67a4d04d70faa7c7a5dc0bb456da38488bbef6a65b3049 }
Xpub slice [4, 53, 135, 207, 2, 209, 224, 227, 59, 0, 0, 0, 0, 247, 42, 221, 220, 211, 91, 75, 25, 63, 103, 164, 208, 77, 112, 250, 167, 199, 165, 220, 11, 180, 86, 218, 56, 72, 139, 190, 246, 166, 91, 48, 73, 2, 77, 212, 117, 147, 32, 139, 1, 218, 49, 190, 153, 92, 127, 107, 128, 180, 168, 230, 1, 170, 186, 171, 147, 225, 30, 134, 65, 41, 240, 142, 92, 84]
Xpub hex "043587cf02d1e0e33b00000000f72adddcd35b4b193f67a4d04d70faa7c7a5dc0bb456da38488bbef6a65b3049024dd47593208b01da31be995c7f6b80b4a8e601aabaab93e11e864129f08e5c54"
Xpriv slice [4, 53, 131, 148, 2, 209, 224, 227, 59, 0, 0, 0, 0, 247, 42, 221, 220, 211, 91, 75, 25, 63, 103, 164, 208, 77, 112, 250, 167, 199, 165, 220, 11, 180, 86, 218, 56, 72, 139, 190, 246, 166, 91, 48, 73, 0, 61, 42, 82, 250, 25, 12, 174, 8, 31, 213, 183, 38, 216, 107, 227, 207, 21, 85, 186, 190, 164, 25, 161, 41, 6, 197, 1, 36, 253, 202, 78, 66]
to sign tpubDBgjffrVpRq8LXetkp5ASKqQVpH2kf9ji9KgP5fkQ4otXx3VyEJM7wXjKYGdGJ8BeyVk7vmgHji6zLtAw4dXdTpFVASuoeGdBpgGohv4Wck0
Serialized signature [31, 16, 221, 78, 67, 151, 246, 235, 208, 214, 239, 190, 163, 57, 48, 99, 224, 109, 255, 42, 162, 80, 85, 198, 41, 128, 210, 183, 253, 80, 242, 156, 146, 31, 225, 79, 90, 131, 61, 23, 139, 225, 91, 118, 144, 199, 176, 225, 78, 247, 8, 5, 84, 215, 7, 91, 191, 186, 28, 241, 163, 181, 5, 51, 93]
is_signed true
verify true
```

In this example "Serialized signature" is the witness byte array and "Xpub slice" is the serialized Xpub byte array associated with the signature.

## Price Data Acquisition

For asset price acquisition please consider examples ws-to-grpc_server.rs and ws-to-grpc_client.rs of this project:
[Scatter-Gather Examples](https://github.com/labormedia/scatter-gather/tree/main/examples)