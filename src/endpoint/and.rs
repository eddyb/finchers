#![allow(clippy::type_complexity)]

use std::fmt;
use std::pin::PinMut;
use std::task;
use std::task::Poll;

use futures_core::future::{Future, TryFuture};
use futures_util::try_future::{TryFutureExt, TryJoin};
use pin_utils::unsafe_pinned;

use crate::common::{Combine, Tuple};
use crate::endpoint::{Context, Endpoint, EndpointResult, IntoEndpoint};
use crate::error::Error;

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub struct And<E1, E2> {
    pub(super) e1: E1,
    pub(super) e2: E2,
}

impl<'a, E1, E2> Endpoint<'a> for And<E1, E2>
where
    E1: Endpoint<'a>,
    E2: Endpoint<'a>,
    E1::Output: Combine<E2::Output>,
{
    type Output = <E1::Output as Combine<E2::Output>>::Out;
    type Future = AndFuture<E1::Future, E2::Future>;

    fn apply(&'a self, ecx: &mut Context<'_>) -> EndpointResult<Self::Future> {
        let f1 = self.e1.apply(ecx)?;
        let f2 = self.e2.apply(ecx)?;
        Ok(AndFuture {
            inner: f1.try_join(f2),
        })
    }
}

pub struct AndFuture<F1, F2>
where
    F1: TryFuture<Error = Error>,
    F2: TryFuture<Error = Error>,
{
    inner: TryJoin<F1, F2>,
}

impl<F1, F2> fmt::Debug for AndFuture<F1, F2>
where
    F1: TryFuture<Error = Error> + fmt::Debug,
    F2: TryFuture<Error = Error> + fmt::Debug,
    F1::Ok: fmt::Debug,
    F2::Ok: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AndFuture")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<F1, F2> AndFuture<F1, F2>
where
    F1: TryFuture<Error = Error>,
    F2: TryFuture<Error = Error>,
{
    unsafe_pinned!(inner: TryJoin<F1, F2>);
}

impl<F1, F2> Future for AndFuture<F1, F2>
where
    F1: TryFuture<Error = Error>,
    F2: TryFuture<Error = Error>,
    F1::Ok: Tuple + Combine<F2::Ok>,
    F2::Ok: Tuple,
{
    type Output = Result<<F1::Ok as Combine<F2::Ok>>::Out, Error>;

    fn poll(mut self: PinMut<'_, Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        self.inner()
            .poll(cx)
            .map_ok(|(v1, v2)| Combine::combine(v1, v2))
    }
}

// ==== tuples ====

impl<'a, E1, E2> IntoEndpoint<'a> for (E1, E2)
where
    E1: IntoEndpoint<'a>,
    E2: IntoEndpoint<'a>,
    E1::Output: Combine<E2::Output>,
{
    type Output = <E1::Output as Combine<E2::Output>>::Out;
    type Endpoint = And<E1::Endpoint, E2::Endpoint>;

    fn into_endpoint(self) -> Self::Endpoint {
        (And {
            e1: self.0.into_endpoint(),
            e2: self.1.into_endpoint(),
        }).with_output::<<E1::Output as Combine<E2::Output>>::Out>()
    }
}

impl<'a, E1, E2, E3> IntoEndpoint<'a> for (E1, E2, E3)
where
    E1: IntoEndpoint<'a>,
    E2: IntoEndpoint<'a>,
    E3: IntoEndpoint<'a>,
    E2::Output: Combine<E3::Output>,
    E1::Output: Combine<<E2::Output as Combine<E3::Output>>::Out>,
{
    type Output = <E1::Output as Combine<<E2::Output as Combine<E3::Output>>::Out>>::Out;
    type Endpoint = And<E1::Endpoint, And<E2::Endpoint, E3::Endpoint>>;

    fn into_endpoint(self) -> Self::Endpoint {
        (And {
            e1: self.0.into_endpoint(),
            e2: And {
                e1: self.1.into_endpoint(),
                e2: self.2.into_endpoint(),
            },
        }).with_output::<<E1::Output as Combine<<E2::Output as Combine<E3::Output>>::Out>>::Out>()
    }
}

impl<'a, E1, E2, E3, E4> IntoEndpoint<'a> for (E1, E2, E3, E4)
where
    E1: IntoEndpoint<'a>,
    E2: IntoEndpoint<'a>,
    E3: IntoEndpoint<'a>,
    E4: IntoEndpoint<'a>,
    E3::Output: Combine<E4::Output>,
    E2::Output: Combine<<E3::Output as Combine<E4::Output>>::Out>,
    E1::Output: Combine<<E2::Output as Combine<<E3::Output as Combine<E4::Output>>::Out>>::Out>,
{
    type Output = <E1::Output as Combine<
        <E2::Output as Combine<<E3::Output as Combine<E4::Output>>::Out>>::Out,
    >>::Out;
    type Endpoint = And<E1::Endpoint, And<E2::Endpoint, And<E3::Endpoint, E4::Endpoint>>>;

    fn into_endpoint(self) -> Self::Endpoint {
        (And {
            e1: self.0.into_endpoint(),
            e2: And {
                e1: self.1.into_endpoint(),
                e2: And {
                    e1: self.2.into_endpoint(),
                    e2: self.3.into_endpoint(),
                },
            },
        }).with_output::<<E1::Output as Combine<
            <E2::Output as Combine<<E3::Output as Combine<E4::Output>>::Out>>::Out,
        >>::Out>()
    }
}

impl<'a, E1, E2, E3, E4, E5> IntoEndpoint<'a> for (E1, E2, E3, E4, E5)
where
    E1: IntoEndpoint<'a>,
    E2: IntoEndpoint<'a>,
    E3: IntoEndpoint<'a>,
    E4: IntoEndpoint<'a>,
    E5: IntoEndpoint<'a>,
    E4::Output: Combine<E5::Output>,
    E3::Output: Combine<<E4::Output as Combine<E5::Output>>::Out>,
    E2::Output: Combine<<E3::Output as Combine<<E4::Output as Combine<E5::Output>>::Out>>::Out>,
    E1::Output: Combine<
        <E2::Output as Combine<
            <E3::Output as Combine<<E4::Output as Combine<E5::Output>>::Out>>::Out,
        >>::Out,
    >,
{
    type Output = <E1::Output as Combine<
        <E2::Output as Combine<
            <E3::Output as Combine<<E4::Output as Combine<E5::Output>>::Out>>::Out,
        >>::Out,
    >>::Out;
    type Endpoint =
        And<E1::Endpoint, And<E2::Endpoint, And<E3::Endpoint, And<E4::Endpoint, E5::Endpoint>>>>;

    fn into_endpoint(self) -> Self::Endpoint {
        (And {
            e1: self.0.into_endpoint(),
            e2: And {
                e1: self.1.into_endpoint(),
                e2: And {
                    e1: self.2.into_endpoint(),
                    e2: And {
                        e1: self.3.into_endpoint(),
                        e2: self.4.into_endpoint(),
                    },
                },
            },
        }).with_output::<<E1::Output as Combine<
            <E2::Output as Combine<
                <E3::Output as Combine<<E4::Output as Combine<E5::Output>>::Out>>::Out,
            >>::Out,
        >>::Out>()
    }
}
