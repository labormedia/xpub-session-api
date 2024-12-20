// Most of this code is inspired or copied from the main Rust Bitcoin Community project 'rust-bitcoin'
// https://github.com/rust-bitcoin/

use std::str::FromStr;
use std::collections::BTreeMap;
use bitcoin::{
    transaction, Address, Amount, Network, OutPoint, Psbt, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Witness,
    bip32::{
        self,
        Fingerprint,
    },
    locktime::absolute,
    taproot::TaprootSpendInfo,
    psbt::{
        Input,
        PsbtSighashType
    },
    secp256k1::Secp256k1,
    key::PublicKey,
    address::{
        error::ParseError,
        NetworkUnchecked,
        NetworkChecked,
    },
    key::FromSliceError,
    consensus::{
        Decodable,
        Encodable,
    },
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize)]
pub struct AddressSerialized {
    address_string: String,
}

impl AddressSerialized {
    pub fn to_address(self, network: Network) -> Result<Address<NetworkChecked>, ParseError> {
        Address::from_str(&self.address_string)?
            .require_network(network)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PublicKeySerialized {
    public_key_slice: Vec<u8>
}

impl PublicKeySerialized {
    pub fn to_public_key(self, ) -> Result<PublicKey, FromSliceError> {
        PublicKey::from_slice(&self.public_key_slice)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PsbtSerialized {
    inputs: Vec<TxIn>,
    out_address_serialized: AddressSerialized,
    pk_change_serialized: PublicKeySerialized,
    spend_amount_u64: u64,
    change_amount_u64: u64,
}

impl PsbtSerialized {
    pub fn try_into_psbt(self) -> Result<Psbt, Box<dyn std::error::Error>> {
        let inputs = self.inputs;
        let out_address = self.out_address_serialized.to_address(Network::Testnet)?;
        let pk_change = self.pk_change_serialized.to_public_key()?;
        let spend_amount = Amount::from_int_btc(self.spend_amount_u64);
        let change_amount = Amount::from_int_btc(self.change_amount_u64);
        Ok(create_ecdsa_psbt(inputs, out_address, pk_change, spend_amount, change_amount)?)
    }
}

pub fn btc_address_from_str(address_str: &str, network: Network) -> Address {
    Address::from_str(address_str).expect("Valid address")
        .require_network(network)
        .expect("Valid Network")

}

pub fn create_ecdsa_psbt(
    inputs: Vec<TxIn>,
    out_address: Address,
    pk_change: PublicKey,
    spend_amount: Amount, 
    change_amount: Amount
) -> Result<Psbt, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    // The spend output is locked to a key controlled by the receiver.
    let spend = TxOut { value: spend_amount, script_pubkey: out_address.script_pubkey() };

    // The change output is locked to a key controlled by us.
    let change = TxOut {
        value: change_amount,
        script_pubkey: ScriptBuf::new_p2wpkh(&pk_change.wpubkey_hash()?), // Change comes back to us.
    };

    // The transaction we want to sign and broadcast.
    let unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP 68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: inputs,                       // Input is 0-indexed.
        output: vec![spend, change],         // Outputs, order does not matter.
    };

    // Now we'll start the PSBT workflow.
    // Step 1: Creator role; that creates,
    // and add inputs and outputs to the PSBT.
    Ok(Psbt::from_unsigned_tx(unsigned_tx)?)
}

pub fn create_psbt_for_taproot_key_path_spend(
    from_address: Address,
    to_address: Address,
    tree: TaprootSpendInfo,
) -> Psbt {
    let send_value = 6400;
    let out_puts = vec![TxOut {
        value: Amount::from_sat(send_value),
        script_pubkey: to_address.script_pubkey(),
    }];
    let prev_tx_id = "06980ca116f74c7845a897461dd0e1d15b114130176de5004957da516b4dee3a";

    let transaction = Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint { txid: prev_tx_id.parse().unwrap(), vout: 0 },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF), // Ignore nSequence.
            witness: Witness::default(),
        }],
        output: out_puts,
    };

    let mut psbt = Psbt::from_unsigned_tx(transaction).unwrap();

    let mfp = "73c5da0a";
    let internal_key_path = "m/86'/1'/0'/0/2";

    let mut origins = BTreeMap::new();
    origins.insert(
        tree.internal_key(),
        (
            vec![],
            (
                mfp.parse::<Fingerprint>().unwrap(),
                internal_key_path.parse::<bip32::DerivationPath>().unwrap(),
            ),
        ),
    );

    let utxo_value = 6588;
    let mut input = Input {
        witness_utxo: {
            let script_pubkey = from_address.script_pubkey();
            Some(TxOut { value: Amount::from_sat(utxo_value), script_pubkey })
        },
        tap_key_origins: origins,
        ..Default::default()
    };
    let ty = "SIGHASH_DEFAULT".parse::<PsbtSighashType>().unwrap();
    input.sighash_type = Some(ty);
    input.tap_internal_key = Some(tree.internal_key());
    input.tap_merkle_root = tree.merkle_root();
    psbt.inputs = vec![input];
    psbt
}

pub fn finalize_psbt_for_key_path_spend(mut psbt: Psbt) -> Psbt {
    psbt.inputs.iter_mut().for_each(|input| {
        let mut script_witness: Witness = Witness::new();
        script_witness.push(input.tap_key_sig.unwrap().to_vec());
        input.final_script_witness = Some(script_witness);
        input.partial_sigs = BTreeMap::new();
        input.sighash_type = None;
        input.redeem_script = None;
        input.witness_script = None;
        input.bip32_derivation = BTreeMap::new();
    });
    psbt
}