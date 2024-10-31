use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::{
    secp256k1,
    bip32::{
        self,
        Xpriv,
        Xpub,
        ChildNumber,
    },
    Address,
    CompressedPublicKey,
    KnownHrp,
    sign_message::{
        MessageSignature,
        signed_msg_hash,
    },
};
use bitcoin_hashes::Hash;

pub fn derive_xpub(init: bip32::Xpub, path: &[u32; 2]) -> bip32::Xpub {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let path = path.map(|x| bip32::ChildNumber::from_normal_idx(x).unwrap());

    init.derive_pub(&secp, &path).unwrap()
}

pub fn derive_address(init: bip32::Xpub, path: &[u32; 2]) -> Address {
    let public_key = derive_xpub(init, path).public_key;
    Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet)
}

pub fn key_pair_from_xpriv<C: secp256k1::Signing + secp256k1::Verification>(
    secp_ctx: &secp256k1::Secp256k1<C>, 
    xpriv: &Xpriv,
    path: &[u32; 2],
) -> (secp256k1::SecretKey, secp256k1::PublicKey) {
    let xpub = Xpub::from_priv(&secp_ctx, &xpriv);
    let child_number = path.map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp_ctx, &child_number).unwrap().public_key;
    let private_key = xpriv.derive_priv(&secp_ctx, &child_number).unwrap().private_key;

    (private_key, public_key)
}

pub fn sign<C: secp256k1::Signing>(
    secp_ctx: &secp256k1::Secp256k1<C>,
    msg: &str,
    privkey: secp256k1::SecretKey,
) -> MessageSignature {
    let msg_hash = signed_msg_hash(msg);
    let msg_to_sign = secp256k1::Message::from_digest(msg_hash.to_byte_array());
    let secp_sig = secp_ctx.sign_ecdsa_recoverable(&msg_to_sign, &privkey);
    MessageSignature { signature: secp_sig, compressed: true }
}