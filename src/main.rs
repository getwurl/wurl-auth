#[macro_use]
extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::process::exit;
use std::io::{Read, stdin};
use hyper::{Client, Method, Request, Uri};
use hyper::client::Response;
use hyper::header::SetCookie;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use clap::App;

fn main() {
    let yaml = load_yaml!("wurl-auth.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let url = value_t_or_exit!(matches, "url", Uri);
    let headers = values_t!(matches, "headers", String).unwrap_or(Vec::new());
    let method = value_t!(matches, "method", Method).unwrap_or(Method::Get);
    let print_headers = matches.is_present("head");
    let mut data = value_t!(matches, "data", String).ok();

    // Read stdin when given --data -
    if let Some(read_data) = data.clone() {
        if read_data == "-" {
            let mut buffer = String::new();
            let stdin = stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer).ok();
            data = Some(buffer);
        }
    }

    let request = build_request(url, method, headers, data);

    match fetch(request, print_headers) {
        Ok(mut response) => {
            if print_headers {
                eprintln!("Authentication response");
                eprintln!("---");
                eprintln!("{} {}", response.version(), response.status());
                eprintln!("{}", response.headers());
            }

            let cookies = response
                .headers_mut()
                .get::<SetCookie>()
                .unwrap_or_else(|| {
                    eprintln!("No Set-Cookie header present");
                    exit(1);
                });

            let mut cookie_values = Vec::new();
            for cookie in cookies.iter() {
                // Get only key=value part of cookie, not the metadata
                let split = cookie.split(';').collect::<Vec<&str>>();
                if let Some(header) = split.first() {
                    cookie_values.push(header.clone());
                }
            }

            print!("Cookie: {}", cookie_values.join("; "));
        }
        Err(error) => eprintln!("An error occured while fetching: {}", error),
    }
}

fn build_request(uri: Uri, method: Method, headers: Vec<String>, data: Option<String>) -> Request {
    let mut request = Request::new(method.clone(), uri.clone());

    for header in headers.iter() {
        let split = header.split(':').collect::<Vec<&str>>();
        request
            .headers_mut()
            .set_raw(split[0].to_owned(), split[1].to_owned());
    }

    if let Some(data) = data {
        request.set_body(data);
    }

    request
}

fn fetch(request: Request, print_headers: bool) -> Result<Response, hyper::Error> {
    let mut core = Core::new()?;

    if print_headers {
        eprintln!("Authentication request");
        eprintln!("---");
        if let Some(query) = request.uri().query() {
            eprintln!(
                "{} {}?{} {}",
                request.method(),
                request.uri().path(),
                query,
                request.version()
            );
        } else {
            eprintln!(
                "{} {} {}",
                request.method(),
                request.uri().path(),
                request.version()
            );
        }
        eprintln!("{}", request.headers());
    }

    let connector = HttpsConnector::new(1, &core.handle())
        .expect("Failed to construct HTTPS connector, exiting");
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    core.run(client.request(request))
}
