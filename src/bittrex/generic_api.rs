//! Use this module to interact with Bittrex through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Bittrex offers.

use bigdecimal::BigDecimal;
use std::str::FromStr;

use exchange::ExchangeApi;
use bittrex::api::BittrexApi;

use error::*;
use types::*;
use bittrex::utils;
use helpers;

impl ExchangeApi for BittrexApi {
    fn ticker(&mut self, pair: Pair) -> Result<Ticker> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_market_summary(pair_name)?;

        let result = utils::parse_result(&raw_response)?;
        let result_array = result.as_array();
        let result_obj = result_array.unwrap()[0].as_object().unwrap();

        let price_str = result_obj.get("Last").unwrap().as_f64().unwrap().to_string();
        let price = BigDecimal::from_str(&price_str).unwrap();

        let ask_str = result_obj.get("Ask").unwrap().as_f64().unwrap().to_string();
        let ask = BigDecimal::from_str(&price_str).unwrap();

        let bid_str = result_obj.get("Bid").unwrap().as_f64().unwrap().to_string();
        let bid = BigDecimal::from_str(&price_str).unwrap();

        let volume_str = result_obj.get("Volume").unwrap().as_f64().unwrap().to_string();
        let vol = BigDecimal::from_str(&price_str).unwrap();

        Ok(Ticker {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            last_trade_price: price,
            lowest_ask: ask,
            highest_bid: bid,
            volume: Some(vol),
        })

    }

    fn orderbook(&mut self, pair: Pair) -> Result<Orderbook> {
        let pair_name = match utils::get_pair_string(&pair) {
            Some(name) => name,
            None => return Err(ErrorKind::PairUnsupported.into()),
        };

        let raw_response = self.get_order_book(pair_name, "both")?;

        let result = utils::parse_result(&raw_response)?;

        let mut ask_offers = Vec::new();    // buy orders
        let mut bid_offers = Vec::new();    // sell orders

        let buy_orders = result["buy"].as_array()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["buy"])))?;

        let sell_orders = result["sell"].as_array()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(format!("{}", result["sell"])))?;

        for ask in buy_orders {
            let ask_obj = ask.as_object().unwrap();

            let price_str = ask_obj.get("Rate").unwrap().as_f64().unwrap().to_string();
            let price = BigDecimal::from_str(&price_str).unwrap();


            let volume_str = ask_obj.get("Quantity").unwrap().as_f64().unwrap().to_string();
            let volume = BigDecimal::from_str(&volume_str).unwrap();

            ask_offers.push((price, volume));
        }

        for bid in sell_orders {
            let bid_obj = bid.as_object().unwrap();

            let price_str = bid_obj.get("Rate").unwrap().as_f64().unwrap().to_string();
            let price = BigDecimal::from_str(&price_str).unwrap();


            let volume_str = bid_obj.get("Quantity").unwrap().as_f64().unwrap().to_string();
            let volume = BigDecimal::from_str(&volume_str).unwrap();

            bid_offers.push((price, volume));
        }

        Ok(Orderbook {
            timestamp: helpers::get_unix_timestamp_ms(),
            pair: pair,
            asks: ask_offers,
            bids: bid_offers,
        })
    }

    fn add_order(&mut self,
                 order_type: OrderType,
                 pair: Pair,
                 quantity: Volume,
                 price: Option<Price>)
                 -> Result<OrderInfo> {
        unimplemented!();
    }

    fn balances(&mut self) -> Result<Balances> {
        unimplemented!();
    }
}
