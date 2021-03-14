//! Default Compute@Edge template program.

use fastly::http::Method;
use fastly::{Error, Request, Response};
use std::collections::HashMap;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND: &str = "jfh_backend";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    if let Ok(q) = req.get_query() {
        let qs_map: HashMap<String, String> = q;
        if let Some((_, nor_val)) = qs_map.get_key_value("nor") {
            println!("Nor: {}", nor_val.to_string());
            let mut nor_req = req.clone_without_body();
            nor_req.set_url(nor_val);
            nor_req.set_method(Method::HEAD);
            nor_req.send_async(BACKEND);
        };
    };

    Ok(req.send(BACKEND)?)
}
