use bitcoin::{
    Address,
    NetworkKind,
    bip32::{
        ChildNumber,
        Xpub,
        Xpriv,
        DerivationPath,
    },
    secp256k1::{
        Secp256k1,
        ffi::types::AlignedType,
    },
    CompressedPublicKey,
    KnownHrp,
};
use rand::Rng;

fn main() {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    println!("Seed {:?}", seed);
    let root_test = Xpriv::new_master(NetworkKind::Test, &seed).unwrap();
    let root_main = Xpriv::new_master(NetworkKind::Main, &seed).unwrap();
    println!("Root key (Testnet): {}", root_test);
    println!("Root key (Mainnet): {}", root_main);

    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let path = "84h/0h/0h".parse::<DerivationPath>().unwrap();
    let child = root_test.derive_priv(&secp, &path).unwrap();
    println!("Child Xpriv: {:?}", child);

    let xpub = Xpub::from_priv(&secp, &child);
    println!("Xpub from child Xpriv: {:?}", xpub);

    let child_number = [0,6].map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp, &child_number).unwrap().public_key;
    let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet);
    println!("Receiving address at m/0/6: {}", address);

    let child_number = [0,0].map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp, &child_number).unwrap().public_key;
    let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet);
    println!("Public key at m/0/0: {}", public_key);
    println!("Receiving address at m/0/0: {}", address);

    let child_number = [0,6].map(|x| ChildNumber::from_normal_idx(x).unwrap());
    let public_key = xpub.derive_pub(&secp, &child_number).unwrap().public_key;
    let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet);
    println!("Public key at m/0/0: {}", public_key);
    println!("Receiving address at m/0/6: {}", address);
}