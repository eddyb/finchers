//! Built-in wrappers.

mod after_apply;
mod and_then;
mod before_apply;
mod map;
mod or_reject;
mod recover;
mod then;
mod try_chain;
mod with_spawner;

pub use self::after_apply::{after_apply, AfterApply};
pub use self::and_then::{and_then, AndThen};
pub use self::before_apply::{before_apply, BeforeApply};
pub use self::map::{map, Map};
pub use self::or_reject::{or_reject, or_reject_with, OrReject, OrRejectWith};
pub use self::recover::{recover, Recover};
pub use self::then::{then, Then};
pub use self::with_spawner::{with_spawner, WithSpawner};

use futures_core::future::{Future, TryFuture};

use crate::common::{Func, Tuple};
use crate::endpoint::Endpoint;
use crate::error::Error;

/// A trait representing the conversion of an endpoint to another endpoint.
pub trait Wrapper<'a, E: Endpoint<'a>> {
    /// The inner type of converted `Endpoint`.
    type Output: Tuple;

    /// The type of converted `Endpoint`.
    type Endpoint: Endpoint<'a, Output = Self::Output>;

    /// Performs conversion from the provided endpoint into `Self::Endpoint`.
    fn wrap(self, endpoint: E) -> Self::Endpoint;
}

#[allow(missing_docs)]
pub trait WrapperExt: Sized {
    fn compose<W>(self, other: W) -> Composed<Self, W> {
        Composed {
            wrapper1: self,
            wrapper2: other,
        }
    }
}

impl<W> WrapperExt for W {}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct Composed<W1, W2> {
    wrapper1: W1,
    wrapper2: W2,
}

impl<'a, E, W1, W2> Wrapper<'a, E> for Composed<W1, W2>
where
    E: Endpoint<'a>,
    W1: Wrapper<'a, E>,
    W2: Wrapper<'a, <W1 as Wrapper<'a, E>>::Endpoint>,
{
    type Output = W2::Output;
    type Endpoint = W2::Endpoint;

    fn wrap(self, endpoint: E) -> Self::Endpoint {
        endpoint
            .wrap(self.wrapper1)
            .wrap(self.wrapper2)
            .with_output::<W2::Output>()
    }
}

/// A set of extension methods for using built-in `Wrapper`s.
pub trait EndpointWrapExt<'a>: Endpoint<'a> + Sized {
    #[allow(missing_docs)]
    fn map<F>(self, f: F) -> <Map<Self::Output, F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Func<Self::Output> + 'a,
    {
        self.wrap(map(f))
    }

    #[allow(missing_docs)]
    fn then<F>(self, f: F) -> <Then<Self::Output, F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Func<Self::Output> + 'a,
        F::Out: Future + 'a,
    {
        self.wrap(then(f))
    }

    #[allow(missing_docs)]
    fn and_then<F>(self, f: F) -> <AndThen<Self::Output, F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Func<Self::Output> + 'a,
        F::Out: TryFuture<Error = Error> + 'a,
    {
        self.wrap(and_then(f))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.12.0-alpha.5",
        note = "use `wrapper::before_apply(f)` instead"
    )]
    fn before_apply<F>(self, f: F) -> <BeforeApply<F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Fn(&mut crate::endpoint::Context<'_>) -> crate::endpoint::EndpointResult<()> + 'a,
    {
        self.wrap(before_apply(f))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.12.0-alpha.5",
        note = "use `wrapper::or_reject()` instead"
    )]
    fn or_reject(self) -> <OrReject as Wrapper<'a, Self>>::Endpoint {
        self.wrap(or_reject())
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.12.0-alpha.5",
        note = "use `wrapper::or_reject_with(f)` instead"
    )]
    fn or_reject_with<F, R>(self, f: F) -> <OrRejectWith<F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Fn(crate::endpoint::EndpointError, &mut crate::endpoint::Context<'_>) -> R + 'a,
        R: Into<crate::error::Error> + 'a,
    {
        self.wrap(or_reject_with(f))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.12.0-alpha.5",
        note = "use `wrapper::recover(f)` instead"
    )]
    fn recover<F, R>(self, f: F) -> <Recover<F> as Wrapper<'a, Self>>::Endpoint
    where
        F: Fn(crate::error::Error) -> R + 'a,
        R: TryFuture<Error = crate::error::Error> + 'a,
    {
        self.wrap(recover(f))
    }
}

impl<'a, E: Endpoint<'a>> EndpointWrapExt<'a> for E {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint;

    #[test]
    fn test_compose() {
        fn assert_impl<E>(endpoint: E)
        where
            for<'a> E: Endpoint<'a>,
        {
            drop(endpoint)
        }

        let w1 = before_apply(|_| Ok(()));
        let w2 = after_apply(|_| Ok(()));
        let wrapper = w1.compose(w2);

        let endpoint = endpoint::unit().wrap(wrapper);
        assert_impl(endpoint);
    }
}
