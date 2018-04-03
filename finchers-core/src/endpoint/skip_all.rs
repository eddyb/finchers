#![allow(missing_docs)]

use futures::future::{ok, FutureResult};
use endpoint::{Endpoint, EndpointContext, IntoEndpoint};
use errors::Error;
use request::Input;

pub fn skip_all<I>(iter: I) -> SkipAll<<I::Item as IntoEndpoint>::Endpoint>
where
    I: IntoIterator,
    I::Item: IntoEndpoint,
{
    SkipAll {
        endpoints: iter.into_iter().map(|e| e.into_endpoint()).collect(),
    }
}

#[derive(Debug, Clone)]
pub struct SkipAll<E: Endpoint> {
    endpoints: Vec<E>,
}

impl<E: Endpoint> Endpoint for SkipAll<E> {
    type Item = ();
    type Future = FutureResult<(), Error>;

    fn apply(&self, input: &Input, ctx: &mut EndpointContext) -> Option<Self::Future> {
        for endpoint in &self.endpoints {
            let _ = endpoint.apply(input, ctx)?;
        }
        Some(ok(()))
    }
}