#![feature(
    pin,
    arbitrary_self_types,
    async_await,
    await_macro,
    futures_api
)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

mod api;
mod database;
mod model;
mod schema;

use failure::Fallible;
use http::StatusCode;
use serde::Deserialize;
use std::env;

use finchers::input::query::Serde;
use finchers::prelude::*;
use finchers::{output, path, routes};

use crate::database::ConnectionPool;

fn main() -> Fallible<()> {
    dotenv::dotenv()?;

    let pool = ConnectionPool::init(env::var("DATABASE_URL")?)?;
    let acquire_conn = endpoint::unit().and_then(move || {
        let fut = pool.acquire_conn();
        async move { await!(fut).map_err(Into::into) }
    });

    let endpoint = path!(/"api"/"v1"/"posts").and(routes!{
        path!(@get /)
            .and(endpoints::query::optional().map(|query: Option<_>| {
                query.map(Serde::into_inner)
            }))
            .and(acquire_conn.clone())
            .and_then(async move |query, conn| await!(crate::api::get_posts(query, conn)).map_err(Into::into))
            .map(output::Json),

        path!(@post /)
            .and(endpoints::body::json())
            .and(acquire_conn.clone())
            .and_then(async move |new_post, conn| await!(crate::api::create_post(new_post, conn)).map_err(Into::into))
            .map(output::Json)
            .map(output::status::Created),

        path!(@get / i32 /)
            .and(acquire_conn.clone())
            .and_then(async move |id, conn| {
                await!(crate::api::find_post(id, conn))?
                    .ok_or_else(|| finchers::error::err_msg(StatusCode::NOT_FOUND, "not found"))
            })
            .map(output::Json),
    });

    finchers::launch(endpoint).start("127.0.0.1:4000");
    Ok(())
}
