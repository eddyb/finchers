#[macro_use]
extern crate finchers;
#[macro_use]
extern crate serde;

use finchers::Endpoint;
use finchers::output::Debug;

fn endpoint() -> impl Endpoint<Item = Debug> + Send + Sync + 'static {
    use finchers::endpoint::abort;
    use finchers::endpoint::prelude::*;
    use finchers::endpoint::query::{from_csv, query, Form};
    use finchers::error::BadRequest;

    #[derive(Debug, Deserialize, HttpStatus)]
    pub struct FormParam {
        query: String,
        count: Option<usize>,
        #[serde(deserialize_with = "from_csv", default)]
        tags: Option<Vec<String>>,
    }

    // Create an endpoint for parsing the form-urlencoded parameter in the request.
    let urlencoded_param = choice![
        // Parse the query string when GET request.
        get(query()),
        // Parse the message body when POST request.
        post(data()).map(Form::into_inner),
        // TODO: add an endpoint for reporting the param error.
        abort(|_| BadRequest::new("Empty parameter")),
    ]
    // annotate to the endpoint that the inner type is FormParam.
    .as_::<FormParam>();

    path("search")
        .right(urlencoded_param)
        .inspect(|param| println!("Received: {:?}", param))
        .map(|param| Debug::new(param).pretty(true))
}

fn main() {
    finchers::run(endpoint());
}
