use exchange::{Exchange, ExchangeApi};
use pair::Pair;
use bitstamp::api::BitstampApi;

use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::value::Map;
use serde_json::value::Value;

#[derive(Debug)]
pub struct Fastcoin;

impl Fastcoin {
    /// Create a new FastcoinApi by providing an API key & API secret
    pub fn new(exchange: Exchange, customer_id: &str, api_key: &str, api_secret: &str) -> Box<ExchangeApi> {

        let mut params = HashMap::new();
        params.insert("customer_id", customer_id);
        params.insert("api_key", api_key);
        params.insert("api_secret", api_secret);

        match exchange {
            Exchange::Bitstamp => Box::new(BitstampApi::new(&params)),
            Exchange::Kraken => Box::new(UnimplementedApi),
            Exchange::Poloniex => Box::new(UnimplementedApi),
        }
    }

    /// Create a new FastcoinApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file(Exchange::Bitstamp, Path::new("/keys.json"))`
    pub fn new_from_file(exchange: Exchange, path: PathBuf) -> Box<ExchangeApi> {

        match exchange {
            Exchange::Bitstamp => Box::new(BitstampApi::new_from_file("account_bitstamp", path)),
            Exchange::Kraken => Box::new(UnimplementedApi),
            Exchange::Poloniex => Box::new(UnimplementedApi),
        }
    }
}


#[derive(Debug)]
struct UnimplementedApi;

impl ExchangeApi for UnimplementedApi {
    fn public_query(&mut self, _: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn private_query(&mut self, _: &HashMap<&str, &str>) -> Option<Map<String, Value>> {
        unimplemented!();
    }

    fn return_ticker(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_trade_history(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_order_book(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
    fn return_balances(&mut self, _: Pair) -> Option<Map<String, Value>> {
        unimplemented!();
    }
}
