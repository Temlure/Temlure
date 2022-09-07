use bidir_map::BidirMap;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error::*;
use pair::Pair;
use pair::Pair::*;
use currency::Currency;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(DASH_EUR, "DASHEUR");
        m.insert(DASH_USD, "DASHUSD");
        m.insert(DASH_BTC, "DASHXBT");
        m.insert(GNO_ETH, "GNOETH");
        m.insert(GNO_EUR, "GNOEUR");
        m.insert(GNO_USD, "GNOUSD");
        m.insert(GNO_BTC, "GNOXBT");
        m.insert(USDT_USD, "USDTZUSD");
        m.insert(ETC_ETH, "XETCXETH");
        m.insert(ETC_BTC, "XETCXXBT");
        m.insert(ETC_EUR, "XETCZEUR");
        m.insert(ETC_USD, "XETCZUSD");
        m.insert(ETH_BTC, "XETHXXBT");
        m.insert(ETH_BTC_d, "XETHXXBT.d");
        m.insert(ETH_CAD, "XETHZCAD");
        m.insert(ETH_CAD_d, "XETHZCAD.d");
        m.insert(ETH_EUR, "XETHZEUR");
        m.insert(ETH_EUR_d, "XETHZEUR.d");
        m.insert(ETH_GBP, "XETHZGBP");
        m.insert(ETH_GBP_d, "XETHZGBP.d");
        m.insert(ETH_JPY, "XETHZJPY");
        m.insert(ETH_JPY_d, "XETHZJPY.d");
        m.insert(ETH_USD, "XETHZUSD");
        m.insert(ETH_USD_d, "XETHZUSD.d");
        m.insert(ICN_ETH, "XICNXETH");
        m.insert(ICN_BTC, "XICNXXBT");
        m.insert(LTC_BTC, "XLTCXXBT");
        m.insert(LTC_EUR, "XLTCZEUR");
        m.insert(LTC_USD, "XLTCZUSD");
        m.insert(MLN_ETH, "XMLNXETH");
        m.insert(MLN_BTC, "XMLNXXBT");
        m.insert(REP_ETH, "XREPXETH");
        m.insert(REP_BTC, "XREPXXBT");
        m.insert(REP_EUR, "XREPZEUR");
        m.insert(REP_USD, "XREPZUSD");
        m.insert(BTC_CAD, "XXBTZCAD");
        m.insert(BTC_CAD_d, "XXBTZCAD.d");
        m.insert(BTC_EUR, "XXBTZEUR");
        m.insert(BTC_EUR_d, "XXBTZEUR.d");
        m.insert(BTC_GBP, "XXBTZGBP");
        m.insert(BTC_GBP_d, "XXBTZGBP.d");
        m.insert(BTC_JPY, "XXBTZJPY");
        m.insert(BTC_JPY_d, "XXBTZJPY.d");
        m.insert(BTC_USD, "XXBTZUSD");
        m.insert(BTC_USD_d, "XXBTZUSD.d");
        m.insert(XDG_BTC, "XXDGXXBT");
        m.insert(XLM_BTC, "XXLMXXBT");
        m.insert(XLM_EUR, "XXLMZEUR");
        m.insert(XLM_USD, "XXLMZUSD");
        m.insert(XMR_BTC, "XXMRXXBT");
        m.insert(XMR_EUR, "XXMRZEUR");
        m.insert(XMR_USD, "XXMRZUSD");
        m.insert(XRP_BTC, "XXRPXXBT");
        m.insert(XRP_CAD, "XXRPZCAD");
        m.insert(XRP_EUR, "XXRPZEUR");
        m.insert(XRP_JPY, "XXRPZJPY");
        m.insert(XRP_USD, "XXRPZUSD");
        m.insert(ZEC_BTC, "XZECXXBT");
        m.insert(ZEC_EUR, "XZECZEUR");
        m.insert(ZEC_USD, "XZECZUSD");
        m
    };
}

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_string(pair: &Pair) -> Option<&&str> {
    PAIRS_STRING.get_by_first(pair)
}

/// Return the Pair enum associated to the string used by Kraken
/// If the Pair is not supported, None is returned.
pub fn get_pair_enum(pair: &str) -> Option<&Pair> {
    PAIRS_STRING.get_by_second(&pair)
}

pub fn deserialize_json(json_string: &str) -> Result<Map<String, Value>> {
    let data: Value = match serde_json::from_str(json_string) {
        Ok(data) => data,
        Err(_) => return Err(ErrorKind::BadParse.into()),
    };

    match data.as_object() {
        Some(value) => Ok(value.clone()),
        None => Err(ErrorKind::BadParse.into()),
    }
}

/// If error array is null, return the result (encoded in a json object)
/// else return the error string found in array
pub fn parse_result(response: &Map<String, Value>) -> Result<Map<String, Value>> {
    let error_array = match response.get("error") {
        Some(array) => {
            array
                .as_array()
                .ok_or_else(|| ErrorKind::InvalidFieldFormat("error".to_string()))?
        }
        None => return Err(ErrorKind::BadParse.into()),
    };
    if error_array.is_empty() {
        return Ok(response
                      .get("result")
                      .ok_or_else(|| ErrorKind::MissingField("result".to_string()))?
                      .as_object()
                      .ok_or_else(|| ErrorKind::InvalidFieldFormat("result".to_string()))?
                      .clone());
    }
    let error_msg = error_array[0]
        .as_str()
        .ok_or_else(|| ErrorKind::InvalidFieldFormat(error_array[0].to_string()))?
        .to_string();

    //TODO: Parse correctly the reason for "EService:Unavailable".
    match error_msg.as_ref() {
        "EService:Unavailable" => {
            Err(ErrorKind::ServiceUnavailable("Unknown...".to_string()).into())
        }
        "EAPI:Invalid key" => Err(ErrorKind::BadCredentials.into()),
        "EAPI:Invalid nonce" => Err(ErrorKind::InvalidNonce.into()),
        "EOrder:Rate limit exceeded" => Err(ErrorKind::RateLimitExceeded.into()),
        "EQuery:Unknown asset pair" => Err(ErrorKind::PairUnsupported.into()),
        "EGeneral:Invalid arguments" => Err(ErrorKind::InvalidArguments.into()),
        "EGeneral:Permission denied" => Err(ErrorKind::PermissionDenied.into()),
        "EOrder:Insufficient funds" => Err(ErrorKind::InsufficientFunds.into()),
        "EOrder:Order minimum not met" => Err(ErrorKind::InsufficientOrderSize.into()),
        other => Err(ErrorKind::ExchangeSpecificError(other.to_string()).into()),
    }
}

/// return None
pub fn get_currency_enum(currency: &str) -> Option<Currency> {
    match currency {
        "ZEUR" => Some(Currency::EUR),
        "ZCAD" => Some(Currency::CAD),
        "ZGBP" => Some(Currency::GBP),
        "ZJPY" => Some(Currency::JPY),
        "ZUSD" => Some(Currency::USD),
        "XDASH" => Some(Currency::DASH),
        "XETC" => Some(Currency::ETC),
        "XETH" => Some(Currency::ETH),
        "XGNO" => Some(Currency::GNO),
        "XICN" => Some(Currency::ICN),
        "XLTC" => Some(Currency::LTC),
        "XMLN" => Some(Currency::MLN),
        "XREP" => Some(Currency::REP),
        "XUSDT" => Some(Currency::USDT),
        "XXBT" => Some(Currency::BTC),
        "XXDG" => Some(Currency::XDG),
        "XXLM" => Some(Currency::XLM),
        "XXMR" => Some(Currency::XMR),
        "XXRP" => Some(Currency::XRP),
        "XZEC" => Some(Currency::ZEC),
        _ => None,
    }
}
