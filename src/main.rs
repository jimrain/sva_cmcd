//! Default Compute@Edge template program.

use fastly::http::{Method};
use fastly::{Error, Request, Response};
use std::collections::HashMap;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND: &str = "jfh_backend";
const BASE_URL: &str = "https://negroni-cmcd.global.ssl.fastly.net";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    println!("sva_nor");

    // Send option requests straight to the backend.
    if req.get_method() == Method::OPTIONS {
        return Ok(req.send(BACKEND)?);
    }

    match req.get_header("cmcd-request") {
        Some(cmcd) => {
            let cmcd = cmcd.to_str().unwrap().to_string();
            send_nor_request(cmcd, &req);
        }
        None => {
            // We looked for a cmcd-request header and it wasn't there so let's see if it's in the
            // query parameters.
            if let Ok(q) = req.get_query() {
                // println!("qp={:?}", q);
                let qs_map: HashMap<String, String> = q;

                if qs_map.contains_key("CMCD") {
                    let cmd = qs_map.get("CMCD").unwrap();
                    send_nor_request(cmd.to_string(), &req);
                }
            };
        }
    }

    Ok(req.send(BACKEND)?)
}

fn send_nor_request(cmcd: String, req: &Request) {
    match get_nor(cmcd) {
        Some(nor) => {
            let nor_len = nor.len() - 1;
            let escaped_nor = &nor[1..nor_len];
            let nor_url = format!("{}{}", BASE_URL, escaped_nor);
            println!("Nor url: {:?}", nor_url);
            let mut nor_req = req.clone_without_body();
            nor_req.set_url(nor_url.as_str());
            nor_req.set_method(Method::HEAD);
            nor_req.send_async(BACKEND);
        }
        None => println!("Nor not found"),
    }
}

fn get_nor(cmcd: String) -> Option<String> {
    let parsed: Vec<&str> = cmcd.split(',').collect();

    let mut kv_map = HashMap::new();
    for v in parsed {
        let i: Vec<&str> = v.split('=').collect();
        if i.len() > 1 {
            kv_map.insert(i[0], i[1]);
        }
    }
    // println!("kv_map: {}", kv_map.get("rid").unwrap());

    let nor = match kv_map.get("nor") {
        Some(n) => Some(n.to_string()),
        None => None,
    };
    nor
}
