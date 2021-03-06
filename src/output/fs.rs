#![allow(missing_docs)]

use std::cmp;
use std::fs::Metadata;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::pin::PinMut;
use std::task;
use std::task::Poll;

use futures_core::future::Future;

use futures::try_ready as try_ready_01;
use futures::Future as Future01;

use tokio::fs::file::{File, MetadataFuture, OpenFuture};
use tokio::prelude::{Async, AsyncRead};

use bytes::{BufMut, Bytes, BytesMut};
use http::{header, Response};
use mime_guess::guess_mime_type;

use super::payload::Payload;
use super::{Output, OutputContext};
use crate::error::Never;

/// An instance of `Responder` representing a file on the file system.
#[derive(Debug)]
pub struct NamedFile {
    file: File,
    meta: Metadata,
    path: PathBuf,
}

impl NamedFile {
    #[allow(missing_docs)]
    pub fn open(path: PathBuf) -> OpenNamedFile {
        OpenNamedFile {
            state: State::Opening(File::open(path.clone())),
            path: Some(path),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct OpenNamedFile {
    state: State,
    path: Option<PathBuf>,
}

#[derive(Debug)]
enum State {
    Opening(OpenFuture<PathBuf>),
    Metadata(MetadataFuture),
    Done,
}

impl OpenNamedFile {
    fn poll_01(&mut self) -> io::Result<Async<NamedFile>> {
        enum Polled {
            Opening(File),
            Metadata((File, Metadata)),
        }

        loop {
            let polled = match self.state {
                State::Opening(ref mut f) => Polled::Opening(try_ready_01!(f.poll())),
                State::Metadata(ref mut f) => Polled::Metadata(try_ready_01!(f.poll())),
                State::Done => panic!("The future cannot poll twice."),
            };

            match (mem::replace(&mut self.state, State::Done), polled) {
                (State::Opening(..), Polled::Opening(file)) => {
                    self.state = State::Metadata(file.metadata());
                }
                (State::Metadata(..), Polled::Metadata((file, meta))) => {
                    let named_file = NamedFile {
                        file,
                        meta,
                        path: self.path.take().unwrap(),
                    };
                    return Ok(Async::Ready(named_file));
                }
                _ => unreachable!("unexpected condition"),
            }
        }
    }
}

impl Future for OpenNamedFile {
    type Output = io::Result<NamedFile>;

    fn poll(self: PinMut<'_, Self>, _: &mut task::Context<'_>) -> Poll<Self::Output> {
        match unsafe { PinMut::get_mut_unchecked(self).poll_01() } {
            Ok(Async::Ready(f)) => Poll::Ready(Ok(f)),
            Ok(Async::NotReady) => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl Output for NamedFile {
    type Body = FileStream;
    type Error = Never;

    fn respond(self, _: &mut OutputContext<'_>) -> Result<Response<Self::Body>, Self::Error> {
        let NamedFile { file, meta, path } = self;

        let body = FileStream::new(file, &meta);

        let content_type = guess_mime_type(&path);

        Ok(Response::builder()
            .header(header::CONTENT_LENGTH, meta.len())
            .header(header::CONTENT_TYPE, content_type.as_ref())
            .body(body)
            .unwrap())
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct FileStream {
    file: File,
    buf: BytesMut,
    buf_size: usize,
    len: u64,
}

impl FileStream {
    fn new(file: File, meta: &Metadata) -> FileStream {
        let buf_size = optimal_buf_size(&meta);
        let len = meta.len();
        FileStream {
            file,
            buf: BytesMut::new(),
            buf_size,
            len,
        }
    }
}

impl Payload for FileStream {
    type Data = io::Cursor<Bytes>;
    type Error = io::Error;

    fn poll_data(&mut self) -> Result<Async<Option<Self::Data>>, Self::Error> {
        if self.len == 0 {
            return Ok(Async::Ready(None));
        }

        if self.buf.remaining_mut() < self.buf_size {
            self.buf.reserve(self.buf_size);
        }

        let n = match try_ready_01!(self.file.read_buf(&mut self.buf)) {
            0 => return Ok(Async::Ready(None)),
            n => n as u64,
        };

        let mut chunk = self.buf.take().freeze();
        if n > self.len {
            chunk = chunk.split_to(self.len as usize);
            self.len = 0;
        } else {
            self.len = n;
        }

        Ok(Async::Ready(Some(io::Cursor::new(chunk))))
    }
}

fn optimal_buf_size(meta: &Metadata) -> usize {
    let blk_size = get_block_size(meta);
    cmp::min(blk_size as u64, meta.len()) as usize
}

#[cfg(unix)]
fn get_block_size(meta: &Metadata) -> usize {
    use std::os::unix::fs::MetadataExt;
    meta.blksize() as usize
}

#[cfg(not(unix))]
fn get_block_size(_: &Metadata) -> usize {
    8192
}
