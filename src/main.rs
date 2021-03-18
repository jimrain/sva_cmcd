//! Default Compute@Edge template program.

use fastly::http::{Method, Url};
use fastly::{Error, Request, Response};
use std::collections::HashMap;
use std::str::FromStr;

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

    if let Ok(q) = req.get_query() {
        // println!("qp={:?}", q);
        let qs_map: HashMap<String, String> = q;

        let cmd = qs_map.get("CMCD").unwrap();
        // println!("qs_map: {}", cmd);

        // let s = one_liner(cmd);

        let mut theString = cmd.to_string().to_owned();

        match get_nor(theString) {
            Some(nor) => {
                println!("Nor: {}", nor);
                let nor_len = nor.len() - 1;
                let escaped_nor = &nor[1..nor_len];
                let nor_url = format!("{}{}", BASE_URL, escaped_nor);
                println!("Nor url: {:?}", nor_url);
                let mut nor_req = req.clone_without_body();
                nor_req.set_url(nor_url.as_str());
                nor_req.set_method(Method::HEAD);
                nor_req.send_async(BACKEND);
            }
            None => println!("None"),
        }
    };

    Ok(req.send(BACKEND)?)
}


fn get_nor(cmcd: String) -> Option<String> {
    let parsed: Vec<&str> = cmcd.split(",").collect();

    let mut kv_map = HashMap::new();
    for v in parsed {
        let i: Vec<&str> = v.split("=").collect();
        if i.len() > 1 {
            kv_map.insert(i[0], i[1]);
        }
    }
    // println!("kv_map: {}", kv_map.get("rid").unwrap());

    let nor = match (kv_map.get("nor")) {
        Some(n) => Some(n.to_string()),
        None => None,
    };
    nor
}
