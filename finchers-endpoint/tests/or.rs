extern crate finchers_endpoint;
extern crate finchers_http;
extern crate finchers_test;

use finchers_endpoint::{just, EndpointExt};
use finchers_http::path::path;
use finchers_test::Client;

#[test]
fn test_or_1() {
    let endpoint = path("foo").right(just("foo")).or(path("bar").right(just("bar")));
    let client = Client::new(endpoint);

    let outcome = client.get("/foo").run();
    assert_eq!(outcome.and_then(Result::ok), Some("foo"));

    let outcome = client.get("/bar").run();
    assert_eq!(outcome.and_then(Result::ok), Some("bar"));
}

#[test]
fn test_or_choose_longer_segments() {
    let e1 = path("foo").right(just("foo"));
    let e2 = path("foo/bar").right(just("foobar"));
    let endpoint = e1.or(e2);
    let client = Client::new(endpoint);

    let outcome = client.get("/foo").run();
    assert_eq!(outcome.and_then(Result::ok), Some("foo"));

    let outcome = client.get("/foo/bar").run();
    assert_eq!(outcome.and_then(Result::ok), Some("foobar"));
}
