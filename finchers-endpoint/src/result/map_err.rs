use finchers_core::endpoint::{Context, Endpoint};
use finchers_core::outcome::{self, Outcome, PollOutcome};

#[derive(Debug, Copy, Clone)]
pub struct MapErr<E, F> {
    endpoint: E,
    f: F,
}

pub fn new<E, F, U, A, B>(endpoint: E, f: F) -> MapErr<E, F>
where
    E: Endpoint<Output = Result<A, B>>,
    F: FnOnce(B) -> U + Clone + Send,
{
    MapErr { endpoint, f }
}

impl<E, F, A, B, U> Endpoint for MapErr<E, F>
where
    E: Endpoint<Output = Result<A, B>>,
    F: FnOnce(B) -> U + Clone + Send,
{
    type Output = Result<A, U>;
    type Outcome = MapErrOutcome<E::Outcome, F>;

    fn apply(&self, cx: &mut Context) -> Option<Self::Outcome> {
        Some(MapErrOutcome {
            outcome: self.endpoint.apply(cx)?,
            f: Some(self.f.clone()),
        })
    }
}

#[derive(Debug)]
pub struct MapErrOutcome<T, F> {
    outcome: T,
    f: Option<F>,
}

impl<T, F, U, A, B> Outcome for MapErrOutcome<T, F>
where
    T: Outcome<Output = Result<A, B>> + Send,
    F: FnOnce(B) -> U + Send,
{
    type Output = Result<A, U>;

    fn poll_outcome(&mut self, cx: &mut outcome::Context) -> PollOutcome<Self::Output> {
        let item = try_poll_outcome!(self.outcome.poll_outcome(cx));
        let f = self.f.take().expect("cannot resolve twice");
        cx.input().enter_scope(|| PollOutcome::Ready(item.map_err(f)))
    }
}