//! Components for accessing of HTTP headers
//!
//! There are three endpoint for accessing the value of HTTP header:
//!
//! * `Header<H, E>` - Returns the value of `H` from the header map. If the value of `H` is not found, then skipping the current route.
//! * `HeaderRequired<H>` - Similar to `Header`, but always matches and returns an error if `H` is not found.
//! * `HeaderOptional<H, E>` - Similar to `Header`, but always matches and returns a `None` if `H` is not found.

use std::fmt;
use std::error::Error;
use std::marker::PhantomData;
use futures::future::{err, ok, FutureResult};
use endpoint::{Endpoint, EndpointContext, EndpointError, EndpointResult};
use http::{header, HttpError, Request, StatusCode};

#[allow(missing_docs)]
pub fn header<H: header::Header>() -> Header<H> {
    Header {
        _marker: PhantomData,
    }
}

#[allow(missing_docs)]
pub struct Header<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H> Copy for Header<H> {}

impl<H> Clone for Header<H> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<H> fmt::Debug for Header<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Header").finish()
    }
}

impl<H: header::Header> Endpoint for Header<H> {
    type Item = H;
    type Result = HeaderResult<H>;

    fn apply(&self, ctx: &mut EndpointContext) -> Option<Self::Result> {
        if ctx.headers().has::<H>() {
            Some(HeaderResult {
                _marker: PhantomData,
            })
        } else {
            None
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct HeaderResult<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H: header::Header> EndpointResult for HeaderResult<H> {
    type Item = H;
    type Future = FutureResult<H, EndpointError>;

    fn into_future(self, request: &mut Request) -> Self::Future {
        ok(request.headers_mut().remove().expect(&format!(
            "The value of header {} has already taken",
            H::header_name()
        )))
    }
}

#[allow(missing_docs)]
pub fn header_req<H: header::Header>() -> HeaderRequired<H> {
    HeaderRequired {
        _marker: PhantomData,
    }
}

#[allow(missing_docs)]
pub struct HeaderRequired<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H> Copy for HeaderRequired<H> {}

impl<H> Clone for HeaderRequired<H> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<H> fmt::Debug for HeaderRequired<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HeaderRequired").finish()
    }
}

impl<H: header::Header> Endpoint for HeaderRequired<H> {
    type Item = H;
    type Result = HeaderRequiredResult<H>;

    fn apply(&self, _: &mut EndpointContext) -> Option<Self::Result> {
        Some(HeaderRequiredResult {
            _marker: PhantomData,
        })
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct HeaderRequiredResult<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H: header::Header> EndpointResult for HeaderRequiredResult<H> {
    type Item = H;
    type Future = FutureResult<H, EndpointError>;

    fn into_future(self, request: &mut Request) -> Self::Future {
        match request.headers_mut().remove() {
            Some(h) => ok(h),
            None => err((EmptyHeader {
                _marker: PhantomData,
            } as EmptyHeader<H>)
                .into()),
        }
    }
}

#[allow(missing_docs)]
pub fn header_opt<H: header::Header>() -> HeaderOptional<H> {
    HeaderOptional {
        _marker: PhantomData,
    }
}

#[allow(missing_docs)]
pub struct HeaderOptional<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H> Copy for HeaderOptional<H> {}

impl<H> Clone for HeaderOptional<H> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<H> fmt::Debug for HeaderOptional<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HeaderOpt").finish()
    }
}

impl<H: header::Header> Endpoint for HeaderOptional<H> {
    type Item = Option<H>;
    type Result = HeaderOptionalResult<H>;

    fn apply(&self, _: &mut EndpointContext) -> Option<Self::Result> {
        Some(HeaderOptionalResult {
            _marker: PhantomData,
        })
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct HeaderOptionalResult<H> {
    _marker: PhantomData<fn() -> H>,
}

impl<H: header::Header> EndpointResult for HeaderOptionalResult<H> {
    type Item = Option<H>;
    type Future = FutureResult<Option<H>, EndpointError>;

    fn into_future(self, request: &mut Request) -> Self::Future {
        ok(request.headers_mut().remove())
    }
}

#[allow(missing_docs)]
pub struct EmptyHeader<H: header::Header> {
    _marker: PhantomData<fn() -> H>,
}

impl<H: header::Header> fmt::Debug for EmptyHeader<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EmptyHeader").finish()
    }
}

impl<H: header::Header> fmt::Display for EmptyHeader<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The header '{}' is not given",
            <H as header::Header>::header_name()
        )
    }
}

impl<H: header::Header> Error for EmptyHeader<H> {
    fn description(&self) -> &str {
        "empty header"
    }
}

impl<H: header::Header> HttpError for EmptyHeader<H> {
    fn status_code(&self) -> StatusCode {
        StatusCode::BadRequest
    }
}

impl<H: header::Header> PartialEq for EmptyHeader<H> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
