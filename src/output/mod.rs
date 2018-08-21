//! Components for constructing HTTP responses.

pub mod fs;
pub mod payload;
pub mod status;

mod binary;
mod debug;
mod json;
mod text;

use http::{Response, StatusCode};
use hyper::body::Payload;
use std::fmt;
use std::mem::PinMut;

use crate::common::Either;
use crate::error::{Error, HttpError, Never};
use crate::input::Input;

use self::payload::Empty;

pub use self::binary::Binary;
pub use self::debug::Debug;
pub use self::fs::NamedFile;
pub use self::json::Json;
pub use self::text::Text;

/// Trait representing types to be converted into an HTTP response.
pub trait Responder {
    /// The type of message body in the HTTP response to the client.
    type Body: Payload;

    /// The error type which will be returned from `respond()`.
    type Error: Into<Error>;

    /// Performs conversion this value into an HTTP response.
    fn respond(self, input: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error>;
}

impl<T: Payload> Responder for Response<T> {
    type Body = T;
    type Error = Never;

    #[inline(always)]
    fn respond(self, _: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        Ok(self)
    }
}

impl Responder for () {
    type Body = Empty;
    type Error = Never;

    fn respond(self, _: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Empty)
            .unwrap())
    }
}

impl<T: Responder> Responder for (T,) {
    type Body = T::Body;
    type Error = T::Error;

    #[inline]
    fn respond(self, input: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        self.0.respond(input)
    }
}

impl<T> Responder for Option<T>
where
    T: Responder,
{
    type Body = T::Body;
    type Error = Error;

    fn respond(self, input: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        self.ok_or_else(|| NoRoute { _priv: () })?
            .respond(input)
            .map_err(Into::into)
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct NoRoute {
    _priv: (),
}

impl fmt::Display for NoRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("no route")
    }
}

impl HttpError for NoRoute {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}

impl<T, E> Responder for Result<T, E>
where
    T: Responder,
    Error: From<E>,
{
    type Body = T::Body;
    type Error = Error;

    fn respond(self, input: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        self?.respond(input).map_err(Into::into)
    }
}

impl<L, R> Responder for Either<L, R>
where
    L: Responder,
    R: Responder,
{
    type Body = Either<L::Body, R::Body>;
    type Error = Error;

    fn respond(self, input: PinMut<'_, Input>) -> Result<Response<Self::Body>, Self::Error> {
        match self {
            Either::Left(l) => l
                .respond(input)
                .map(|res| res.map(Either::Left))
                .map_err(Into::into),
            Either::Right(r) => r
                .respond(input)
                .map(|res| res.map(Either::Right))
                .map_err(Into::into),
        }
    }
}
