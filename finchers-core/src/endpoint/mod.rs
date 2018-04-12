mod context;
mod error;
pub mod task;

use std::rc::Rc;
use std::sync::Arc;

// re-exports
pub use self::context::{Context, Segment, Segments};
pub use self::error::{Error, ErrorKind};
pub use self::task::Task;

/// Trait representing an *endpoint*.
pub trait Endpoint {
    /// The inner type associated with this endpoint.
    type Item;

    /// The type of asynchronous "Task" which will be returned from "apply".
    type Task: Task<Output = Self::Item> + Send;

    /// Perform checking the incoming HTTP request and returns
    /// an instance of the associated task if matched.
    fn apply(&self, cx: &mut Context) -> Option<Self::Task>;

    /// Ensure that the associated type `Item` is equal to `T`.
    #[inline(always)]
    fn with_item_type<T>(self) -> Self
    where
        Self: Sized + Endpoint<Item = T>,
    {
        self
    }
}

impl<'a, E: Endpoint> Endpoint for &'a E {
    type Item = E::Item;
    type Task = E::Task;

    fn apply(&self, cx: &mut Context) -> Option<Self::Task> {
        (*self).apply(cx)
    }
}

impl<E: Endpoint> Endpoint for Box<E> {
    type Item = E::Item;
    type Task = E::Task;

    fn apply(&self, cx: &mut Context) -> Option<Self::Task> {
        (**self).apply(cx)
    }
}

impl<E: Endpoint> Endpoint for Rc<E> {
    type Item = E::Item;
    type Task = E::Task;

    fn apply(&self, cx: &mut Context) -> Option<Self::Task> {
        (**self).apply(cx)
    }
}

impl<E: Endpoint> Endpoint for Arc<E> {
    type Item = E::Item;
    type Task = E::Task;

    fn apply(&self, cx: &mut Context) -> Option<Self::Task> {
        (**self).apply(cx)
    }
}

/// Abstruction of types to be convert to an `Endpoint`.
pub trait IntoEndpoint {
    /// The return type
    type Item;
    /// The type of value returned from `into_endpoint`.
    type Endpoint: Endpoint<Item = Self::Item>;

    /// Convert itself into `Self::Endpoint`.
    fn into_endpoint(self) -> Self::Endpoint;
}

impl<E: Endpoint> IntoEndpoint for E {
    type Item = E::Item;
    type Endpoint = E;

    #[inline]
    fn into_endpoint(self) -> Self::Endpoint {
        self
    }
}
