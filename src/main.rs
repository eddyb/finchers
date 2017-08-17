extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;

pub mod endpoint;
pub mod responder;
pub mod input;

use futures::{Future, Stream};
use hyper::Get;
use tokio_core::reactor::Core;

use endpoint::{Endpoint, param};
use input::Input;
use responder::Responder;

fn main() {
    let endpoint = param("hello");

    let input = Input::new(Get, "/?hello=world");
    println!("input: {:#?}", input);
    println!();

    if let Ok(f) = endpoint.apply(input) {
        let mut core = Core::new().unwrap();
        let output = core.run(f.map(|r| r.respond()));

        match output {
            Ok(response) => {
                println!("success: {:#?}", response);
                let body = core.run(
                    response
                        .body()
                        .map_err(|_| ())
                        .fold(Vec::new(), |mut body, chunk| {
                            body.extend_from_slice(&chunk);
                            Ok(body)
                        })
                        .and_then(|body| String::from_utf8(body).map_err(|_| ())),
                );
                println!("..with body: {:?}", body);
            }
            Err(err) => eprintln!("failed with: {:?}", err),
        }
    } else {
        eprintln!("no route");
    }
}
