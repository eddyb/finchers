#![allow(missing_docs)]

use std::marker::PhantomData;
use super::{Endpoint, EndpointContext};

pub fn ok<T: Clone, E>(x: T) -> EndpointOk<T, E> {
    EndpointOk {
        x,
        _marker: PhantomData,
    }
}

#[derive(Debug)]
pub struct EndpointOk<T: Clone, E> {
    x: T,
    _marker: PhantomData<fn() -> E>,
}

impl<T: Clone, E> Endpoint for EndpointOk<T, E> {
    type Item = T;
    type Error = E;
    type Task = Result<T, E>;

    fn apply(&self, _: &mut EndpointContext) -> Option<Self::Task> {
        Some(Ok(self.x.clone()))
    }
}

pub fn err<T, E: Clone>(x: E) -> EndpointErr<T, E> {
    EndpointErr {
        x,
        _marker: PhantomData,
    }
}

#[derive(Debug)]
pub struct EndpointErr<T, E: Clone> {
    x: E,
    _marker: PhantomData<fn() -> T>,
}

impl<T, E: Clone> Endpoint for EndpointErr<T, E> {
    type Item = T;
    type Error = E;
    type Task = Result<T, E>;

    fn apply(&self, _: &mut EndpointContext) -> Option<Self::Task> {
        Some(Err(self.x.clone()))
    }
}

pub fn result<T: Clone, E: Clone>(x: Result<T, E>) -> EndpointResult<T, E> {
    EndpointResult { x }
}

#[derive(Debug)]
pub struct EndpointResult<T: Clone, E: Clone> {
    x: Result<T, E>,
}

impl<T: Clone, E: Clone> Endpoint for EndpointResult<T, E> {
    type Item = T;
    type Error = E;
    type Task = Result<T, E>;

    fn apply(&self, _: &mut EndpointContext) -> Option<Self::Task> {
        Some(self.x.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::TestRunner;
    use hyper::{Method, Request};

    #[test]
    fn test_ok() {
        let endpoint = ok("Alice");
        let mut runner = TestRunner::new(endpoint).unwrap();
        let request = Request::new(Method::Get, "/".parse().unwrap());
        let result: Option<Result<&str, Result<(), _>>> = runner.run(request);
        match result {
            Some(Ok("Alice")) => (),
            _ => panic!("does not match"),
        }
    }

    #[test]
    fn test_err() {
        let endpoint = err("Alice");
        let mut runner = TestRunner::new(endpoint).unwrap();
        let request = Request::new(Method::Get, "/".parse().unwrap());
        let result: Option<Result<(), Result<&str, _>>> = runner.run(request);
        match result {
            Some(Err(Ok("Alice"))) => (),
            _ => panic!("does not match"),
        }
    }

    #[test]
    fn test_result() {
        let endpoint = result(Ok("Alice"));
        let mut runner = TestRunner::new(endpoint).unwrap();
        let request = Request::new(Method::Get, "/".parse().unwrap());
        let result: Option<Result<&str, Result<(), _>>> = runner.run(request);
        match result {
            Some(Ok("Alice")) => (),
            _ => panic!("does not match"),
        }
    }
}
