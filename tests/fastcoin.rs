#[cfg(test)]
mod fastcoin_tests {
    extern crate fastcoin;

    use self::fastcoin::fastcoin::Fastcoin;
    use self::fastcoin::exchange::{Exchange, ExchangeApi};
    use self::fastcoin::pair::Pair;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api: Box<ExchangeApi> = Fastcoin::new(Exchange::Bitstamp,
                                                  "bs_api_key",
                                                  "bs_api_secret",
                                                  Some("bs_cust_id"))
                .unwrap();

        assert_eq!(format!("{:?}", api),
                   "BitstampApi { last_request: 0, api_key: \"bs_api_key\", api_secret: \
                    \"bs_api_secret\", customer_id: \"bs_cust_id\", http_client: Client { \
                    redirect_policy: FollowAll, read_timeout: None, write_timeout: None, proxy: \
                    None } }");
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
        //        let api = Fastcoin::new(Exchange::Kraken, "", "", "");
        //        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
        //        let api = Fastcoin::new(Exchange::Poloniex, "", "", "");
        //        assert_eq!(api, Exchange::Poloniex);
    }

    #[test]
    fn fastcoin_can_get_a_ticker_from_bitstamp() {
        let mut api = Fastcoin::new(Exchange::Bitstamp,
                                    "bs_api_key",
                                    "bs_api_secret",
                                    Some("bs_cust_id"))
                .unwrap();
        let ticker = api.ticker(Pair::BTC_USD);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    #[test]
    fn fastcoin_can_get_a_ticker_from_kraken() {
        let mut api = Fastcoin::new(Exchange::Kraken, "api_key", "api_secret", None).unwrap();
        let ticker = api.ticker(Pair::BTC_EUR);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    #[test]
    fn fastcoin_can_get_a_ticker_from_poloniex() {
        let mut api = Fastcoin::new(Exchange::Poloniex, "api_key", "api_secret", None).unwrap();
        let ticker = api.ticker(Pair::BTC_ETH);

        assert_ne!(ticker.unwrap().last_trade_price, 0.0);
    }

    // IMPORTANT: Real keys are needed in order to retrieve the balance
    #[test]
    #[cfg_attr(not(feature = "bitstamp_private_tests"), ignore)]
    fn balance_should_have_usd_btc_fee() {
        use std::path::PathBuf;
        let path = PathBuf::from("./keys_real.json");
        let mut api = Fastcoin::new_from_file(Exchange::Bitstamp, "account_bitstamp", path)
            .unwrap();
        let result = api.return_balances(Pair::BTC_USD).unwrap();
        let result_looking_for_usd = result.clone();
        let result_looking_for_btc = result.clone();
        let result_looking_for_fee = result.clone();

        assert!(result_looking_for_usd.contains_key("usd_balance"));
        assert!(result_looking_for_btc.contains_key("btc_balance"));
        assert!(result_looking_for_fee.contains_key("fee"));
    }
}
