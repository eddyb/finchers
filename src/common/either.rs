#![allow(missing_docs)]

use std::error::Error as StdError;
use std::fmt;
use std::pin::PinMut;

use futures;
use futures_core::future::Future;
use futures_core::task;
use futures_core::task::Poll;

use bytes::Buf;
use http::header::HeaderMap;
use hyper::body::Payload;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    #[inline]
    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(ref t) => Either::Left(t),
            Either::Right(ref t) => Either::Right(t),
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(ref mut t) => Either::Left(t),
            Either::Right(ref mut t) => Either::Right(t),
        }
    }

    #[inline]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention)]
    pub fn as_pin_mut<'a>(self: PinMut<'a, Self>) -> Either<PinMut<'a, L>, PinMut<'a, R>> {
        match unsafe { PinMut::get_mut_unchecked(self) } {
            Either::Left(ref mut t) => Either::Left(unsafe { PinMut::new_unchecked(t) }),
            Either::Right(ref mut t) => Either::Right(unsafe { PinMut::new_unchecked(t) }),
        }
    }
}

impl<T> Either<T, T> {
    pub fn into_inner(self) -> T {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L: Buf, R: Buf> Buf for Either<L, R> {
    fn remaining(&self) -> usize {
        match self {
            Either::Left(ref t) => t.remaining(),
            Either::Right(ref t) => t.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        match self {
            Either::Left(ref t) => t.bytes(),
            Either::Right(ref t) => t.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        match self {
            Either::Left(ref mut t) => t.advance(cnt),
            Either::Right(ref mut t) => t.advance(cnt),
        }
    }
}

impl<L: Payload, R: Payload> Payload for Either<L, R> {
    type Data = Either<L::Data, R::Data>;
    type Error = Box<dyn StdError + Send + Sync>;

    fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
        match self {
            Either::Left(ref mut t) => t
                .poll_data()
                .map(|x| x.map(|data| data.map(Either::Left)))
                .map_err(Into::into),
            Either::Right(ref mut t) => t
                .poll_data()
                .map(|x| x.map(|data| data.map(Either::Right)))
                .map_err(Into::into),
        }
    }

    fn poll_trailers(&mut self) -> futures::Poll<Option<HeaderMap>, Self::Error> {
        match self {
            Either::Left(ref mut t) => t.poll_trailers().map_err(Into::into),
            Either::Right(ref mut t) => t.poll_trailers().map_err(Into::into),
        }
    }

    fn is_end_stream(&self) -> bool {
        match self {
            Either::Left(ref t) => t.is_end_stream(),
            Either::Right(ref t) => t.is_end_stream(),
        }
    }

    fn content_length(&self) -> Option<u64> {
        match self {
            Either::Left(ref t) => t.content_length(),
            Either::Right(ref t) => t.content_length(),
        }
    }
}

impl<L, R> fmt::Display for Either<L, R>
where
    L: fmt::Display,
    R: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(ref t) => t.fmt(formatter),
            Either::Right(ref t) => t.fmt(formatter),
        }
    }
}

impl<L, R> StdError for Either<L, R>
where
    L: StdError,
    R: StdError,
{
    fn description(&self) -> &str {
        match self {
            Either::Left(ref t) => t.description(),
            Either::Right(ref t) => t.description(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Either::Left(ref t) => t.cause(),
            Either::Right(ref t) => t.cause(),
        }
    }
}

impl<L, R> Future for Either<L, R>
where
    L: Future,
    R: Future,
{
    type Output = Either<L::Output, R::Output>;

    fn poll(self: PinMut<'_, Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        match self.as_pin_mut() {
            Either::Left(t) => t.poll(cx).map(Either::Left),
            Either::Right(t) => t.poll(cx).map(Either::Right),
        }
    }
}
