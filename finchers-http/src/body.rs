//! Components for parsing an HTTP request body.

use bytes::Bytes;
use futures::Future;
use std::marker::PhantomData;
use std::str::Utf8Error;
use std::{error, fmt};

use finchers_core::endpoint::{Context, Endpoint};
use finchers_core::error::BadRequest;
use finchers_core::input;
use finchers_core::task::{self, PollTask, Task};
use finchers_core::{BytesString, Input};

/// Creates an endpoint for parsing the incoming request body into the value of `T`
pub fn body<T: FromBody>() -> Body<T> {
    Body { _marker: PhantomData }
}

#[allow(missing_docs)]
pub struct Body<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for Body<T> {}

impl<T> Clone for Body<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Body<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Body").finish()
    }
}

impl<T: FromBody> Endpoint for Body<T> {
    type Item = T;
    type Task = BodyTask<T>;

    fn apply(&self, cx: &mut Context) -> Option<Self::Task> {
        match T::is_match(cx.input()) {
            true => Some(BodyTask::Init),
            false => None,
        }
    }
}

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub enum BodyTask<T> {
    Init,
    Recv(input::Body),
    Done(PhantomData<fn() -> T>),
}

impl<T: FromBody> Task for BodyTask<T> {
    type Output = T;

    fn poll_task(&mut self, cx: &mut task::Context) -> PollTask<Self::Output> {
        'poll: loop {
            let next = match *self {
                BodyTask::Init => {
                    let body = cx.input_mut().body().expect("The body has already taken");
                    BodyTask::Recv(body.into_data())
                }
                BodyTask::Recv(ref mut body) => {
                    let buf = try_ready!(body.poll());
                    let body = T::from_body(buf, cx.input_mut()).map_err(BadRequest::new)?;
                    return Ok(body.into());
                }
                _ => panic!("cannot resolve/reject twice"),
            };
            *self = next;
        }
    }
}

/// Creates an endpoint for taking the instance of `BodyStream`
pub fn body_stream() -> BodyStream {
    BodyStream { _priv: () }
}

#[allow(missing_docs)]
pub struct BodyStream {
    _priv: (),
}

impl Copy for BodyStream {}

impl Clone for BodyStream {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for BodyStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BodyStream").finish()
    }
}

impl Endpoint for BodyStream {
    type Item = input::BodyStream;
    type Task = BodyStreamTask;

    fn apply(&self, _: &mut Context) -> Option<Self::Task> {
        Some(BodyStreamTask { _priv: () })
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct BodyStreamTask {
    _priv: (),
}

impl Task for BodyStreamTask {
    type Output = input::BodyStream;

    fn poll_task(&mut self, cx: &mut task::Context) -> PollTask<Self::Output> {
        let body = cx.input_mut().body().expect("cannot take a body twice");
        Ok(input::BodyStream::from(body).into())
    }
}

/// The conversion from received request body.
pub trait FromBody: 'static + Sized {
    /// The type of error value returned from `from_body`.
    type Error: error::Error + Send + 'static;

    /// Returns whether the incoming request matches to this type or not.
    ///
    /// This method is used only for the purpose of changing the result of routing.
    /// Otherwise, use `validate` instead.
    #[allow(unused_variables)]
    fn is_match(input: &Input) -> bool {
        true
    }

    /// Performs conversion from raw bytes into itself.
    fn from_body(body: Bytes, input: &mut Input) -> Result<Self, Self::Error>;
}

impl FromBody for () {
    type Error = !;

    fn from_body(_: Bytes, _: &mut Input) -> Result<Self, Self::Error> {
        Ok(())
    }
}

impl FromBody for Bytes {
    type Error = !;

    fn from_body(body: Bytes, _: &mut Input) -> Result<Self, Self::Error> {
        Ok(body)
    }
}

impl FromBody for BytesString {
    type Error = Utf8Error;

    fn from_body(body: Bytes, _: &mut Input) -> Result<Self, Self::Error> {
        BytesString::from_shared(body)
    }
}

impl FromBody for String {
    type Error = Utf8Error;

    fn from_body(body: Bytes, _: &mut Input) -> Result<Self, Self::Error> {
        BytesString::from_shared(body).map(Into::into)
    }
}

impl<T: FromBody> FromBody for Option<T> {
    type Error = !;

    fn from_body(body: Bytes, input: &mut Input) -> Result<Self, Self::Error> {
        Ok(T::from_body(body, input).ok())
    }
}

impl<T: FromBody> FromBody for Result<T, T::Error> {
    type Error = !;

    fn from_body(body: Bytes, input: &mut Input) -> Result<Self, Self::Error> {
        Ok(T::from_body(body, input))
    }
}
