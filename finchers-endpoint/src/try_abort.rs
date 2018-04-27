use finchers_core::HttpError;
use finchers_core::endpoint::{Context, Endpoint};
use finchers_core::outcome::{self, Outcome, PollOutcome};

pub fn new<E, F, T, R>(endpoint: E, f: F) -> TryAbort<E, F>
where
    E: Endpoint,
    F: FnOnce(E::Output) -> Result<T, R> + Clone + Send,
    R: HttpError,
{
    TryAbort { endpoint, f }
}

#[derive(Copy, Clone, Debug)]
pub struct TryAbort<E, F> {
    endpoint: E,
    f: F,
}

impl<E, F, T, R> Endpoint for TryAbort<E, F>
where
    E: Endpoint,
    F: FnOnce(E::Output) -> Result<T, R> + Clone + Send,
    R: HttpError,
{
    type Output = T;
    type Outcome = TryAbortOutcome<E::Outcome, F>;

    fn apply(&self, cx: &mut Context) -> Option<Self::Outcome> {
        Some(TryAbortOutcome {
            outcome: self.endpoint.apply(cx)?,
            f: Some(self.f.clone()),
        })
    }
}

#[derive(Debug)]
pub struct TryAbortOutcome<T, F> {
    outcome: T,
    f: Option<F>,
}

impl<T, F, U, E> Outcome for TryAbortOutcome<T, F>
where
    T: Outcome + Send,
    F: FnOnce(T::Output) -> Result<U, E> + Clone + Send,
    E: HttpError,
{
    type Output = U;

    fn poll_outcome(&mut self, cx: &mut outcome::Context) -> PollOutcome<Self::Output> {
        let item = try_poll_outcome!(self.outcome.poll_outcome(cx));
        let f = self.f.take().expect("cannot resolve/reject twice");
        cx.input().enter_scope(|| match f(item) {
            Ok(item) => PollOutcome::Ready(item),
            Err(err) => PollOutcome::Abort(Into::into(err)),
        })
    }
}
