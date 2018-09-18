#![feature(rust_2018_preview, futures_api, pin, arbitrary_self_types)]

extern crate finchers;
extern crate futures;
extern crate http; // 0.3
extern crate pin_utils;

use pin_utils::unsafe_pinned;
use std::pin::PinMut;

use futures::future::{Future, TryFuture};
use futures::task;
use futures::task::Poll;

use finchers::endpoint::{Context, Endpoint, EndpointError, EndpointResult, Wrapper};
use finchers::error::Error;
use finchers::input::with_get_cx;
use finchers::output::{Output, OutputContext};

use http::header::HeaderValue;
use http::header::{
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD,
    ORIGIN,
};
use http::Method;

pub fn cors_filter() -> CorsFilter {
    CorsFilter { _priv: () }
}

#[derive(Debug)]
pub struct CorsFilter {
    _priv: (),
}

impl<'a, E: Endpoint<'a>> Wrapper<'a, E> for CorsFilter {
    type Output = (CorsResponse<E::Output>,);
    type Endpoint = CorsEndpoint<E>;

    fn wrap(self, endpoint: E) -> Self::Endpoint {
        CorsEndpoint { endpoint }
    }
}

/// An endpoint which represents a route with CORS handling.
///
/// The value of this type is generated by `CorsFilter::wrap()`.
#[derive(Debug)]
pub struct CorsEndpoint<E> {
    endpoint: E,
}

impl<'a, E> Endpoint<'a> for CorsEndpoint<E>
where
    E: Endpoint<'a>,
{
    type Output = (CorsResponse<E::Output>,);
    type Future = CorsFuture<'a, E>;

    fn apply(&'a self, cx: &mut Context) -> EndpointResult<Self::Future> {
        Ok(CorsFuture {
            inner: self.endpoint.apply(cx).map_err(Some),
            endpoint: self,
        })
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct CorsFuture<'a, E: Endpoint<'a>> {
    inner: Result<E::Future, Option<EndpointError>>,
    endpoint: &'a CorsEndpoint<E>,
}

impl<'a, E> CorsFuture<'a, E>
where
    E: Endpoint<'a>,
{
    unsafe_pinned!(inner: Result<E::Future, Option<EndpointError>>);
}

impl<'a, E> Future for CorsFuture<'a, E>
where
    E: Endpoint<'a>,
{
    type Output = Result<(CorsResponse<E::Output>,), Error>;

    fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        match unsafe { PinMut::get_mut_unchecked(self.inner()) } {
            Ok(ref mut future) => {
                let future = unsafe { PinMut::new_unchecked(future) };
                with_get_cx(|input| {
                    if input.headers().get(ORIGIN).is_none() {
                        return future
                            .try_poll(cx)
                            .map_ok(|output| (CorsResponse::Raw(output),));
                    }

                    match *input.method() {
                        Method::OPTIONS => {
                            // preflight
                            let allow_method = if let Some(m) =
                                input.headers().get(ACCESS_CONTROL_REQUEST_METHOD)
                            {
                                m
                            } else {
                                return future
                                    .try_poll(cx)
                                    .map_ok(|output| (CorsResponse::Normal(output),));
                            };
                            let allow_headers = input.headers().get(ACCESS_CONTROL_REQUEST_HEADERS);

                            let response = CorsResponse::Preflight {
                                allow_method: allow_method.clone(),
                                allow_headers: allow_headers.cloned(),
                            };
                            
                            Poll::Ready(Ok((response,)))
                        }
                        _ => {
                            // normal CORS handling without preflight
                            future
                                .try_poll(cx)
                                .map_ok(|output| (CorsResponse::Normal(output),))
                        }
                    }
                })
            }
            Err(ref mut err) => Poll::Ready(Err(err.take().unwrap().into())),
        }
    }
}

#[derive(Debug)]
pub enum CorsResponse<T> {
    Raw(T),
    Normal(T),
    Preflight {
        allow_method: HeaderValue,
        allow_headers: Option<HeaderValue>,
    },
}

impl<T: Output> Output for CorsResponse<T> {
    type Body = T::Body;
    type Error = T::Error;

    fn respond(self, cx: &mut OutputContext) -> Result<http::Response<Self::Body>, Self::Error> {
        match self {
            CorsResponse::Raw(output) => output.respond(cx),
            CorsResponse::Normal(output) => {
                let mut response = output.respond(cx)?;
                response
                    .headers_mut()
                    .entry(ACCESS_CONTROL_ALLOW_ORIGIN)
                    .unwrap()
                    .or_insert_with(|| HeaderValue::from_static("*"));
                Ok(response)
            }
            CorsResponse::Preflight { .. } => unimplemented!(),
        }
    }
}
