use bitcoin::{
    Address,
    CompressedPublicKey,
    secp256k1::{
        self,
        SecretKey,
        PublicKey,
        Secp256k1,
        ffi::types::AlignedType,
    },
    sign_message::signed_msg_hash,
    NetworkKind,
    bip32::{
        ChildNumber,
        Xpub,
        Xpriv,
    },
};
use rand::Rng;

use xpub_session_api::model::derivation::{
    sign,
    verify,
};

fn key_pair_from_xpriv<C: secp256k1::Signing + secp256k1::Verification>(
    secp_ctx: &secp256k1::Secp256k1<C>, 
    xpriv: &Xpriv,
    path: &[u32; 2],
) -> (Xpub, SecretKey, PublicKey) {
    let xpub = Xpub::from_priv(secp_ctx, xpriv);
    println!("Xpub {}", xpub);
    let child_number = path.map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let xpub_child = xpub.derive_pub(secp_ctx, &child_number).unwrap();
    let xpub_slice = xpub_child.encode();
    let public_key = xpub_child.public_key;
    println!("Xpub child string {:?}", xpub_child.to_string());
    println!("Xpub child {:?}", xpub_child);
    println!("Xpub child slice {:?}", xpub_slice);
    println!("Xpub child hex {:?}", hex::encode(xpub_slice));

    let xpriv_child = xpriv.derive_priv(secp_ctx, &child_number).unwrap();
    println!("Xpriv child {}", xpriv_child);
    println!("Xpriv child slice {:?}", xpriv_child.encode());
    let private_key = xpriv_child.private_key;

    (xpub_child, private_key, public_key)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nonce_str = std::env::args().nth(1).expect("Expected nonce number.");

    println!("nonce: {:?}", nonce_str);

    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let xpriv = Xpriv::new_master(NetworkKind::Test, &seed).unwrap();
    println!("xpriv: {}", xpriv);
    println!("xpriv key: {}", xpriv);

    let xpub = Xpub::from_priv(&secp, &xpriv);
    let (xpub_child, private_key, public_key) = key_pair_from_xpriv(&secp, &xpriv, &[0,0]);

    let mut to_sign = xpub_child.to_string().to_owned();
    to_sign.push_str(&nonce_str);
    println!("to sign {}", to_sign);
    let signature = sign(&secp, &to_sign, private_key);
    println!("Serialized signature {:?}", signature.serialize());

    let address = Address::p2pkh(CompressedPublicKey(public_key), NetworkKind::Test);
    let message_hash = signed_msg_hash(&to_sign);
    let is_signed = signature.is_signed_by_address(&secp, &address, message_hash)?;
    println!("is_signed {}", is_signed);

    let verify = verify(public_key, &to_sign, signature)?;
    println!("verify {}", verify);

    Ok(())
}