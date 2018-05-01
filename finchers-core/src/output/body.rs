use bytes::Bytes;
use futures::Async::*;
use futures::{Poll, Stream};
use std::{fmt, io};

pub struct ResponseBody {
    inner: Inner,
}

enum Inner {
    Empty,
    Once(Option<Bytes>),
    Stream(Box<Stream<Item = Bytes, Error = io::Error> + Send>),
}

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            Inner::Empty => f.debug_tuple("Empty").finish(),
            Inner::Once(ref bytes) => f.debug_tuple("Once").field(bytes).finish(),
            Inner::Stream(..) => f.debug_tuple("Stream").finish(),
        }
    }
}

impl Default for ResponseBody {
    fn default() -> ResponseBody {
        ResponseBody::empty()
    }
}

impl From<()> for ResponseBody {
    fn from(_: ()) -> ResponseBody {
        ResponseBody::empty()
    }
}

macro_rules! impl_from_once {
    ($($t:ty),*) => {$(
        impl From<$t> for ResponseBody {
            fn from(body: $t) -> ResponseBody {
                ResponseBody::once(body)
            }
        }
    )*};
}

impl_from_once!(&'static str, String, &'static [u8], Vec<u8>, Bytes);

impl ResponseBody {
    pub fn empty() -> ResponseBody {
        ResponseBody { inner: Inner::Empty }
    }

    pub fn once<T>(body: T) -> ResponseBody
    where
        T: Into<Bytes>,
    {
        ResponseBody {
            inner: Inner::Once(Some(body.into())),
        }
    }

    pub fn wrap_stream<T>(stream: T) -> ResponseBody
    where
        T: Stream<Item = Bytes, Error = io::Error> + Send + 'static,
    {
        ResponseBody {
            inner: Inner::Stream(Box::new(stream)),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self.inner {
            Inner::Empty => Some(0),
            Inner::Once(ref chunk) => chunk.as_ref().map(|c| c.len()),
            Inner::Stream(..) => None,
        }
    }
}

impl Stream for ResponseBody {
    type Item = Bytes;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.inner {
            Inner::Empty => Ok(Ready(None)),
            Inner::Once(ref mut chunk) => Ok(Ready(chunk.take())),
            Inner::Stream(ref mut stream) => stream.poll(),
        }
    }
}
