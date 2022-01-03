use std::path::PathBuf;

extern crate fastcoin;

use fastcoin::kraken::KrakenApi;

fn main() {
    // We create a KrakenApi by providing API key/secret
    let path = PathBuf::from("keys_real.json");
    let mut my_api = KrakenApi::new_from_file("account_kraken", path);

    println!("return {:?}", my_api.get_account_balance());
}
