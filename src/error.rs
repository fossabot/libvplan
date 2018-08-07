use super::http;
use super::hyper;
use super::serde_xml_rs;
use std::fmt;
use std::string;

#[derive(Debug)]
/// An error during the retrieval of a vplan via the client
pub enum RequestError {
    /// Passed an invalid day (Saturday or Sunday)
    InvalidDay,
    /// Error parsing URL
    URLParsingError(http::uri::InvalidUri),
    /// Error during parsing body (bytes) to string
    BodyParsingError(string::FromUtf8Error),
    /// Error during parsing the XML response
    XMLParsingError(ParsingError),
    /// An error from the http crate
    Http(http::Error),
    /// An error from the hyper crate
    Hyper(hyper::Error)
}

#[derive(Debug)]
/// An error occuring during parsing
pub enum ParsingError {
    /// Error parsing actual XML
    /// This might indicate something on the original documents changed.
    DocumentParsingError(serde_xml_rs::Error),
    /// Error indicating a failure to parse dates from the XML
    DateParsingError(String)
}

impl fmt::Display for RequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::InvalidDay => write!(formatter, "passed invalid day"),
            RequestError::URLParsingError(ref error) => {
                write!(formatter, "error parsing URL: {}", error)
            },
            RequestError::BodyParsingError(ref error) => {
                write!(formatter, "error parsing HTTP response body: {}", error)
            },
            RequestError::XMLParsingError(ref error) => write!(formatter, "{}", error),
            RequestError::Http(ref error) => write!(formatter, "{}", error),
            RequestError::Hyper(ref error) => write!(formatter, "{}", error)
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::DocumentParsingError(error) => {
                write!(formatter, "error parsing XML document: {}", error)
            },
            ParsingError::DateParsingError(error) => {
                write!(formatter, "error parsing vplan date: {}", error)
            },
        }
    }
}

impl From<hyper::Error> for RequestError {
    fn from(error: hyper::Error) -> Self {
        RequestError::Hyper(error)
    }
}

impl From<http::Error> for RequestError {
    fn from(error: http::Error) -> Self {
        RequestError::Http(error)
    }
}
