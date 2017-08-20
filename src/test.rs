//! Helper functions for testing

use std::cell::RefCell;
use hyper::Method;
use hyper::error::UriError;
use hyper::header::Header;
use tokio_core::reactor::Core;

use context::Context;
use endpoint::Endpoint;
use errors::*;
use request::{Request, Body};

#[allow(missing_docs)]
pub struct TestCase {
    pub request: Request,
    pub body: Option<Body>,
}

#[allow(missing_docs)]
impl TestCase {
    pub fn new(method: Method, uri: &str) -> Result<Self, UriError> {
        let request = Request::new(method, uri)?;
        Ok(Self {
            request,
            body: None,
        })
    }

    pub fn get(uri: &str) -> Result<Self, UriError> {
        Self::new(Method::Get, uri)
    }

    pub fn post(uri: &str) -> Result<Self, UriError> {
        Self::new(Method::Post, uri)
    }

    pub fn put(uri: &str) -> Result<Self, UriError> {
        Self::new(Method::Put, uri)
    }

    pub fn delete(uri: &str) -> Result<Self, UriError> {
        Self::new(Method::Delete, uri)
    }

    pub fn patch(uri: &str) -> Result<Self, UriError> {
        Self::new(Method::Patch, uri)
    }

    pub fn with_header<H: Header>(mut self, header: H) -> Self {
        self.request.headers.set(header);
        self
    }

    pub fn with_body<B: Into<Body>>(mut self, body: B) -> Self {
        self.body = Some(body.into());
        self
    }
}


/// Invoke given endpoint and return its result
pub fn run_test<E: Endpoint>(endpoint: E, input: TestCase) -> Result<E::Item, FinchersError> {
    let req = input.request;
    let body = RefCell::new(Some(input.body.unwrap_or_default()));
    let ctx = Context::new(&req, &body);

    let (_ctx, f) = endpoint.apply(ctx);
    let f = f?;

    let mut core = Core::new().unwrap();
    core.run(f)
}
