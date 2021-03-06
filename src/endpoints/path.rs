//! Components for parsing request path

use std::fmt;
use std::marker::PhantomData;

use failure::Fail;
use futures_util::future;
use http::StatusCode;
use percent_encoding::{define_encode_set, percent_encode, DEFAULT_ENCODE_SET};

use crate::endpoint::{Context, Endpoint, EndpointError, EndpointResult};
use crate::error::{Error, HttpError};
use crate::input::FromEncodedStr;

define_encode_set! {
    /// The encode set for MatchPath
    #[doc(hidden)]
    pub MATCH_PATH_ENCODE_SET = [DEFAULT_ENCODE_SET] | {'/'}
}

// ==== MatchPath =====

pub fn path(s: impl AsRef<str>) -> MatchPath {
    let s = s.as_ref();
    debug_assert!(!s.is_empty());
    (MatchPath {
        encoded: percent_encode(s.as_bytes(), MATCH_PATH_ENCODE_SET).to_string(),
    }).with_output::<()>()
}

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MatchPath {
    encoded: String,
}

impl<'a> Endpoint<'a> for MatchPath {
    type Output = ();
    type Future = future::Ready<Result<Self::Output, Error>>;

    fn apply(&self, ecx: &mut Context<'_>) -> EndpointResult<Self::Future> {
        let s = ecx.next_segment().ok_or_else(EndpointError::not_matched)?;
        if s == self.encoded {
            Ok(future::ready(Ok(())))
        } else {
            Err(EndpointError::not_matched())
        }
    }
}

// ==== EndPath ====

/// Create an endpoint to check if the path has reached the end.
#[inline]
pub fn end() -> EndPath {
    (EndPath { _priv: () }).with_output::<()>()
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub struct EndPath {
    _priv: (),
}

impl<'a> Endpoint<'a> for EndPath {
    type Output = ();
    type Future = future::Ready<Result<Self::Output, Error>>;

    fn apply(&self, ecx: &mut Context<'_>) -> EndpointResult<Self::Future> {
        match ecx.next_segment() {
            None => Ok(future::ready(Ok(()))),
            Some(..) => Err(EndpointError::not_matched()),
        }
    }
}

// ==== Param ====

#[inline]
pub fn param<T>() -> Param<T>
where
    T: FromEncodedStr,
{
    (Param {
        _marker: PhantomData,
    }).with_output::<(T,)>()
}

#[allow(missing_docs)]
pub struct Param<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for Param<T> {}

impl<T> Clone for Param<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Param<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Param").finish()
    }
}

impl<'a, T> Endpoint<'a> for Param<T>
where
    T: FromEncodedStr,
{
    type Output = (T,);
    type Future = future::Ready<Result<Self::Output, Error>>;

    fn apply(&self, ecx: &mut Context<'_>) -> EndpointResult<Self::Future> {
        let s = ecx.next_segment().ok_or_else(EndpointError::not_matched)?;
        let result = T::from_encoded_str(s)
            .map(|x| (x,))
            .map_err(|cause| ParamError { cause }.into());
        Ok(future::ready(result))
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ParamError<E> {
    cause: E,
}

impl<E: fmt::Display> fmt::Display for ParamError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse a path segment: {}", self.cause)
    }
}

impl<E: Fail> HttpError for ParamError<E> {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

// ==== Remains ====

#[inline]
pub fn remains<T>() -> Remains<T>
where
    T: FromEncodedStr,
{
    (Remains {
        _marker: PhantomData,
    }).with_output::<(T,)>()
}

#[allow(missing_docs)]
pub struct Remains<T> {
    _marker: PhantomData<fn() -> (T)>,
}

impl<T> Copy for Remains<T> {}

impl<T> Clone for Remains<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Remains<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Remains").finish()
    }
}

impl<'a, T> Endpoint<'a> for Remains<T>
where
    T: FromEncodedStr,
{
    type Output = (T,);
    type Future = future::Ready<Result<Self::Output, Error>>;

    fn apply(&self, ecx: &mut Context<'_>) -> EndpointResult<Self::Future> {
        let result = T::from_encoded_str(ecx.remaining_path())
            .map(|x| (x,))
            .map_err(|cause| ParamError { cause }.into());
        while let Some(..) = ecx.next_segment() {}
        Ok(future::ready(result))
    }
}
