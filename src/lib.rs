//! ![Fastcoin](https://raw.githubusercontent.com/hugues31/fastcoin/master/fastcoin.png)
//!
//! Fastcoin is a Rust library aiming to provide a complete access to REST APIs for various
//! crypto-currencies exchanges (see below for a list of supported exchanges).
//! All methods consume HTTPS api. TThe purpose of this crate is not
//! to stream data (you should use websocket/FIX in that case).
//!
//! For optional parameters, enter an empty &str ("") if you don't specify it.
//!
//! ### Exchange support:
//! - [x] Poloniex
//! - [x] Kraken
//! - [x] Bitstamp (partial)
//!
//! # WARNING
//! This library is highly experimental at the moment. Please do not invest what you
//! can't afford to loose. This is a personal project, I can not be held responsible for
//! the library malfunction, which can lead to a loss of money.

#[macro_use]
extern crate hyper;
extern crate crypto;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;

pub mod exchange;
pub mod bitstamp;
pub mod poloniex;
pub mod kraken;
mod helpers;

#[cfg(test)]
mod tests {}
