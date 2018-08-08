use super::base64;
use super::chrono::Weekday;
use super::error::RequestError;
use super::futures::{future, Future, Poll, Stream};
use super::http;
use super::hyper;
use super::hyper_rustls;
use super::parser;
use super::vplan;
use std::boxed::Box;

/// A client to retrieve a vplan
///
/// # Example
/// ```rust,ignore
/// extern crate chrono;
/// extern crate futures;
/// extern crate libvplan;
/// extern crate tokio;
///
/// use chrono::Weekday;
/// use futures::Future;
/// use libvplan::Client;
///
/// let client = Client::new("username", "password");
///
/// let future = client
///                 .get_vplan(Weekday::Mon)
///                 .and_then(|vplan| {
///                     println!("{:#?}", vplan);
///                     Ok(())
///                 })
///                 .map_err(|error| {
///                     panic!("{}", error);
///                 });
///
/// tokio::run(future);
/// ```
pub struct Client {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    authorization: String
}

/// A `Future` that will resolve to a vplan or an error during fetching it
pub struct ResponseFuture {
    inner: Box<Future<Item = vplan::Vplan, Error = RequestError> + Send>
}

impl ResponseFuture {
    fn new(future: Box<Future<Item = vplan::Vplan, Error = RequestError> + Send>) -> Self {
        Self { inner: future }
    }
}

impl Future for ResponseFuture {
    type Item = vplan::Vplan;
    type Error = RequestError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl Client {
    /// Creates a new client
    pub fn new<S: ToString>(username: S, password: S) -> Client {
        let username = username.to_string();
        let password = password.to_string();

        let connector = hyper_rustls::HttpsConnector::new(1);

        Self {
            client: hyper::Client::builder().keep_alive(true).build(connector),
            authorization: format!(
                "Basic {}",
                base64::encode(format!("{}:{}", username, password).as_bytes())
            )
        }
    }

    /// Retrieves the vplan for the given weekday
    pub fn get_vplan(self, day: Weekday) -> ResponseFuture {
        let day = match day {
            Weekday::Mon => Some("Mo"),
            Weekday::Tue => Some("Di"),
            Weekday::Wed => Some("Mi"),
            Weekday::Thu => Some("Do"),
            Weekday::Fri => Some("Fr"),
            _ => None
        };

        if day.is_none() {
            return ResponseFuture::new(Box::new(future::err(RequestError::InvalidDay)));
        }

        let day = day.unwrap();

        let url = format!("https://www.fssgym.de/vplan/tag_{}.xml", day);
        let uri = url.parse::<hyper::Uri>();

        if let Err(error) = uri {
            return ResponseFuture::new(Box::new(future::err(RequestError::URLParsingError(error))));
        }

        let uri = uri.unwrap();

        let request = hyper::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header(http::header::AUTHORIZATION, self.authorization.as_str())
            .body(hyper::Body::empty());

        if let Err(error) = request {
            return ResponseFuture::new(Box::new(future::err(RequestError::Http(error))));
        }

        let request = request.unwrap();

        ResponseFuture::new(Box::new(
            self.client
                .request(request)
                .and_then(|response| {
                    let (parts, body) = response.into_parts();
                    body.concat2().map(|body| (parts, body))
                }).from_err()
                .and_then(move |(_, body)| {
                    let body = String::from_utf8(body.to_vec());

                    if let Err(error) = body {
                        return Err(RequestError::BodyParsingError(error));
                    }

                    let body = body.unwrap();

                    match parser::parse(body.as_ref()) {
                        Ok(vplan) => Ok(vplan),
                        Err(error) => Err(RequestError::XMLParsingError(error))
                    }
                })
        ))
    }
}
