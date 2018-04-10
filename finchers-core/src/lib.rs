//! A combinator library for building asynchronous HTTP services.

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate http;
extern crate mime;

#[cfg(feature = "from_hyper")]
extern crate hyper;

mod never;
mod string;

pub mod endpoint;
pub mod error;
pub mod input;
pub mod output;
pub mod util;

// re-exports
pub use bytes::Bytes;
pub use error::HttpError;
pub use input::Input;
pub use never::Never;
pub use output::Output;
pub use string::BytesString;
