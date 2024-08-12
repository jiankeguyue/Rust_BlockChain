use blockchain1_rust::{BlockChain, SledDb};
use std::env::current_dir;
fn main() {
    tracing_subscriber::fmt().init();
    let path = current_dir().unwrap().join("data");
    let mut bc = BlockChain::new(SledDb::new(path));

    // bc.mine_block("trade: Bob get 2 btc");
    // bc.mine_block("trade: bod sell 1 btc");

    bc.show_block();
}