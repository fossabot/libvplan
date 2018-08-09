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
extern crate hyper_rustls;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

/// The client to fetch a vplan
pub mod client;
mod document;
/// Errors...
pub mod error;
/// Parser for dates.
pub mod parser;
/// A simple representation of a vplan, without `chrono` dates
pub mod simple;
/// vplan
pub mod vplan;

pub use client::Client;
pub use vplan::*;
