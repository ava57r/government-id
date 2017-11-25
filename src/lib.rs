extern crate chrono;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate xml;
extern crate xmltree;

pub mod checkers;
pub mod error;
pub mod fns_service;
#[cfg(test)]
mod unit_test;

pub type ValidResult = std::result::Result<bool, error::Error>;
