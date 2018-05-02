use bytes::Bytes;
use futures::Async::*;
use futures::future::{self, FutureResult};
use futures::{self, Future};
use http::StatusCode;
use http::header::{self, HeaderValue};
use http::{Request, Response};
use std::sync::Arc;
use std::{fmt, io};

use finchers_core::endpoint::ApplyRequest;
use finchers_core::error::Error;
use finchers_core::input::RequestBody;
use finchers_core::output::{Responder, ResponseBody};
use finchers_core::{Endpoint, HttpError, Input, Poll, Task};

use service::{HttpService, NewHttpService, Payload};

impl Payload for ResponseBody {
    type Data = Bytes;
    type Error = io::Error;

    fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
        self.poll_data().into()
    }
}

/// An HTTP service which wraps an `Endpoint`.
pub struct NewEndpointService<E> {
    endpoint: Arc<E>,
    error_handler: ErrorHandler,
}

impl<E> NewEndpointService<E> {
    pub fn new(endpoint: E) -> NewEndpointService<E> {
        NewEndpointService {
            endpoint: Arc::new(endpoint),
            error_handler: default_error_handler,
        }
    }

    pub fn set_error_handler(&mut self, handler: ErrorHandler) {
        self.error_handler = handler;
    }
}

impl<E> NewHttpService for NewEndpointService<E>
where
    E: Endpoint,
    E::Output: Responder,
{
    type RequestBody = RequestBody;
    type ResponseBody = ResponseBody;
    type Error = io::Error;
    type Service = EndpointService<E>;
    type InitError = io::Error;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(EndpointService {
            endpoint: self.endpoint.clone(),
            error_handler: self.error_handler,
            _priv: (),
        })
    }
}

pub struct EndpointService<E> {
    endpoint: Arc<E>,
    error_handler: ErrorHandler,
    _priv: (),
}

impl<E> HttpService for EndpointService<E>
where
    E: Endpoint,
    E::Output: Responder,
{
    type RequestBody = RequestBody;
    type ResponseBody = ResponseBody;
    type Error = io::Error;
    type Future = EndpointServiceFuture<E::Task>;

    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future {
        let (parts, body) = request.into_parts();
        let input = Input::new(Request::from_parts(parts, ()));
        let apply = self.endpoint.apply_request(&input, body);

        EndpointServiceFuture {
            apply,
            input,
            error_handler: self.error_handler,
        }
    }
}

#[allow(missing_debug_implementations)]
pub struct EndpointServiceFuture<T> {
    apply: ApplyRequest<T>,
    input: Input,
    error_handler: ErrorHandler,
}

impl<T> EndpointServiceFuture<T> {
    fn handle_error(&self, err: &HttpError) -> Response<ResponseBody> {
        (self.error_handler)(err, &self.input)
    }
}

impl<T> Future for EndpointServiceFuture<T>
where
    T: Task,
    T::Output: Responder,
{
    type Item = Response<ResponseBody>;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let mut response = match self.apply.poll_ready(&self.input) {
            Poll::Pending => return Ok(NotReady),
            Poll::Ready(Some(Ok(output))) => output
                .respond(&self.input)
                .unwrap_or_else(|err| self.handle_error(Into::<Error>::into(err).as_http_error())),
            Poll::Ready(Some(Err(err))) => self.handle_error(err.as_http_error()),
            Poll::Ready(None) => self.handle_error(&NoRoute),
        };

        if !response.headers().contains_key(header::SERVER) {
            response.headers_mut().insert(
                header::SERVER,
                HeaderValue::from_static(concat!("finchers-runtime/", env!("CARGO_PKG_VERSION"))),
            );
        }

        Ok(Ready(response))
    }
}

#[derive(Debug)]
struct NoRoute;

impl fmt::Display for NoRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("no route")
    }
}

impl HttpError for NoRoute {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}

///
pub type ErrorHandler = fn(&HttpError, &Input) -> Response<ResponseBody>;

fn default_error_handler(err: &HttpError, _: &Input) -> Response<ResponseBody> {
    let body = err.to_string();
    let body_len = body.len().to_string();

    let mut response = Response::new(ResponseBody::once(body));
    *response.status_mut() = err.status_code();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response.headers_mut().insert(header::CONTENT_LENGTH, unsafe {
        HeaderValue::from_shared_unchecked(body_len.into())
    });
    err.append_headers(response.headers_mut());

    response
}