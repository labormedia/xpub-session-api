use bitcoin::{
    Address,
    CompressedPublicKey,
    KnownHrp,
    secp256k1::{
        self,
        SecretKey,
        PublicKey,
        Secp256k1,
        ffi::types::AlignedType,
        Verification,
    },
    sign_message::{
        MessageSignature,
        signed_msg_hash,
    },
    hashes::{sha256d, HashEngine},
    NetworkKind,
    bip32::{
        ChildNumber,
        Xpub,
        Xpriv,
        DerivationPath,
    },
};
use bitcoin_hashes::Hash;
use rand::Rng;

pub fn sign<C: secp256k1::Signing>(
    secp_ctx: &secp256k1::Secp256k1<C>,
    msg: &str,
    privkey: SecretKey,
) -> MessageSignature {
    let msg_hash = signed_msg_hash(msg);
    let msg_to_sign = secp256k1::Message::from_digest(msg_hash.to_byte_array());
    let secp_sig = secp_ctx.sign_ecdsa_recoverable(&msg_to_sign, &privkey);
    MessageSignature { signature: secp_sig, compressed: true }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let message_hash = signed_msg_hash("hello world");
    println!("Signed message: {}", message_hash);

    let (secret_key, public_key) = random_signer(&secp);
    let signature: MessageSignature = sign(&secp, "hello world", secret_key);

    println!("{:?}", signature);

    //let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Testnets);
    let address = Address::p2pkh(&CompressedPublicKey(public_key), NetworkKind::Test);

    let is_signed = signature.is_signed_by_address(&secp, &address, message_hash)?;
    println!("is_signed {}", is_signed);
    Ok(())
}

fn random_signer<C: secp256k1::Signing + secp256k1::Verification>(
    secp_ctx: &secp256k1::Secp256k1<C>,
) -> (SecretKey, PublicKey) 
{
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let root_test = Xpriv::new_master(NetworkKind::Test, &seed).unwrap();
    let path = "84h/0h/0h".parse::<DerivationPath>().unwrap();
    let priv_child = root_test.derive_priv(&secp_ctx, &path).unwrap();
    
    // Private
    let secret_key = priv_child.private_key;
    let xpub_child = Xpub::from_priv(&secp_ctx, &priv_child);

    // Public
    let xpub = Xpub::from_priv(&secp_ctx, &priv_child);
    let child_number = [0,0].map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp_ctx, &child_number).unwrap().public_key;
    let private_key = priv_child.derive_priv(&secp_ctx, &child_number).unwrap().private_key;

    (private_key, public_key)
}

fn key_pair_from_xpriv<C: secp256k1::Signing + secp256k1::Verification>(
    secp_ctx: &secp256k1::Secp256k1<C>, 
    xpriv: &Xpriv,
    path: &[u32; 2],
) -> (SecretKey, PublicKey) {
    let xpub = Xpub::from_priv(&secp_ctx, &xpriv);
    let child_number = path.map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp_ctx, &child_number).unwrap().public_key;
    let private_key = xpriv.derive_priv(&secp_ctx, &child_number).unwrap().private_key;

    (private_key, public_key)
}