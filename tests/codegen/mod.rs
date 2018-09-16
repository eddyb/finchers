use finchers::prelude::*;
use finchers_codegen::endpoint;

use finchers::local;
use matches::assert_matches;

#[endpoint]
fn foo() -> (u32,) {
    endpoint::value(42)
}

#[test]
fn smoketest_endpoint() {
    let endpoint = foo().with_output::<(u32,)>();

    assert_matches!(
        local::get("/")
            .apply(&endpoint),
        Ok((ref val,)) if *val == 42
    );
}
