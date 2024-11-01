extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    let rpc = Client::new("http://localhost:18443",
                          Auth::UserPass("project".to_string(),
                                         "xpub_manager".to_string())).unwrap();
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {}", best_block_hash);

    
}