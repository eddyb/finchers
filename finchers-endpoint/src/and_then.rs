use super::chain::Chain;
use finchers_core::HttpError;
use finchers_core::endpoint::{Context, Endpoint, Error};
use futures::{Future, IntoFuture, Poll};

pub fn new<E, F, R>(endpoint: E, f: F) -> AndThen<E, F>
where
    E: Endpoint,
    F: FnOnce(E::Item) -> R + Clone + Send,
    R: IntoFuture,
    R::Future: Send,
    R::Error: HttpError,
{
    AndThen { endpoint, f }
}

#[derive(Copy, Clone, Debug)]
pub struct AndThen<E, F> {
    endpoint: E,
    f: F,
}

impl<E, F, R> Endpoint for AndThen<E, F>
where
    E: Endpoint,
    F: FnOnce(E::Item) -> R + Clone + Send,
    R: IntoFuture,
    R::Future: Send,
    R::Error: HttpError,
{
    type Item = R::Item;
    type Future = AndThenFuture<E::Future, F, R>;

    fn apply(&self, cx: &mut Context) -> Option<Self::Future> {
        let future = self.endpoint.apply(cx)?;
        Some(AndThenFuture {
            inner: Chain::new(future, self.f.clone()),
        })
    }
}

#[derive(Debug)]
pub struct AndThenFuture<T, F, R>
where
    T: Future<Error = Error>,
    F: FnOnce(T::Item) -> R + Send,
    R: IntoFuture,
    R::Future: Send,
    R::Error: HttpError,
{
    inner: Chain<T, R::Future, F>,
}

impl<T, F, R> Future for AndThenFuture<T, F, R>
where
    T: Future<Error = Error>,
    F: FnOnce(T::Item) -> R + Send,
    R: IntoFuture,
    R::Future: Send,
    R::Error: HttpError,
{
    type Item = R::Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll(|result, f| match result {
            Ok(item) => Ok(Err(f(item).into_future())),
            Err(err) => Err(err),
        })
    }
}
