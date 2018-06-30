//! # libvplan - specific document parsing
//!
//! An internally used library for utilizing a special case of document parsing
//!

extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod client;
mod document;
pub mod error;
pub mod parser;
pub mod vplan;

pub use client::Client;
