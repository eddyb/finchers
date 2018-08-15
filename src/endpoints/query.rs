//! Components for parsing the query string and urlencoded payload.

use std::future::Future;
use std::marker::PhantomData;
use std::mem::PinMut;
use std::task::Poll;
use std::{fmt, task};

use failure::format_err;

use crate::endpoint::Endpoint;
use crate::error::{bad_request, Error};
use crate::generic::{one, One};
use crate::input::query::{FromQuery, QueryItems};
use crate::input::{with_get_cx, Cursor, Input};

/// Create an endpoint which parse the query string in the HTTP request
/// to the value of `T`.
///
/// # Example
///
/// ```
/// # #![feature(rust_2018_preview)]
/// # #![feature(use_extern_macros)]
/// # extern crate finchers;
/// # extern crate serde;
/// # use finchers::endpoints::path::path;
/// # use finchers::endpoints::query::{query};
/// # use finchers::endpoint::EndpointExt;
/// # use finchers::input::query::{from_csv, Serde};
/// # use serde::Deserialize;
/// #
/// #[derive(Debug, Deserialize)]
/// pub struct Param {
///     query: String,
///     count: Option<u32>,
///     #[serde(deserialize_with = "from_csv", default)]
///     tags: Vec<String>,
/// }
///
/// let endpoint = path("foo").and(query())
///     .map(|param: Serde<Param>| (format!("Received: {:?}", &*param),));
/// ```
pub fn query<T>() -> Query<T>
where
    T: FromQuery,
{
    Query {
        _marker: PhantomData,
    }
}

#[allow(missing_docs)]
pub struct Query<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for Query<T> {}

impl<T> Clone for Query<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Query").finish()
    }
}

impl<T> Endpoint for Query<T>
where
    T: FromQuery,
{
    type Output = One<T>;
    type Future = QueryFuture<T>;

    fn apply<'c>(
        &self,
        _: PinMut<'_, Input>,
        cursor: Cursor<'c>,
    ) -> Option<(Self::Future, Cursor<'c>)> {
        Some((
            QueryFuture {
                _marker: PhantomData,
            },
            cursor,
        ))
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct QueryFuture<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Future for QueryFuture<T>
where
    T: FromQuery,
{
    type Output = Result<One<T>, Error>;

    fn poll(self: PinMut<'_, Self>, _: &mut task::Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(with_get_cx(|input| match input.request().uri().query() {
            Some(query) => T::from_query(QueryItems::new(query))
                .map(one)
                .map_err(bad_request),
            None => Err(bad_request(format_err!(
                "The query string is not exist in the request"
            ))),
        }))
    }
}