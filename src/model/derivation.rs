use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::{
    bip32,
    Address,
    CompressedPublicKey,
    KnownHrp,
};

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