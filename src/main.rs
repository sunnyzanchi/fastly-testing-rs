//! Default Compute@Edge template program.

use fastly::http::Url;
use fastly::{Error, Request, Response};

const EN_BACKEND: &str = "docs_en";
const KR_BACKEND: &str = "docs_kr";
const JP_BACKEND: &str = "docs_jp";
/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    req.set_header("Custom-Header", "example");
    let mut url = Url::from(req.get_url().clone());
    url.set_port(Some(443)).unwrap();
    req.set_url(url);

    let path = req.get_path();
    println!("requesting url {}", req.get_url_str());

    let backend;
    if path.contains("kr/") {
        backend = KR_BACKEND;
    } else if path.contains("jp/") {
        backend = JP_BACKEND;
    } else {
        backend = EN_BACKEND;
    }
    println!("using {} backend", backend);

    let result = req.send(backend);
    match result {
        Ok(res) => {
            println!("request succeeded");
            println!("{}", format!("{:#?}", res));
            Ok(res)
        }
        Err(error) => {
            println!("request failed with error: {}", error);
            Err(error.into())
        }
    }
}
