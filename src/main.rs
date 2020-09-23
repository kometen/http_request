extern crate base64;
extern crate clap;
extern crate hyper;

use clap::{Arg, App};
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Method, Request};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("http_request")
    .version("0.1")
    .about("parse pause")
    .author("Claus Guttesen")
    .arg(Arg::with_name("location")
        .help("location of url")
        .required(true)
        .takes_value(true)
        .short("l")
        .long("location")
        .multiple(false)
    )
    .arg(Arg::with_name("username")
        .help("username")
        .required(true)
        .takes_value(true)
        .short("u")
        .long("username")
        .multiple(false)
    )
    .arg(Arg::with_name("password")
        .help("password")
        .required(true)
        .takes_value(true)
        .short("p")
        .long("password")
        .multiple(false)
    )
    .get_matches();

    println!("Jeg æder blåbærsyltetøj!");

    let url = matches.value_of("location").unwrap();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    let b64 = format!("Basic {}", base64::encode(format!("{}:{}", username, password)));
    println!("{}", b64);
    println!("{}", url);

    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Authorization", b64)
        .body(Body::from(""))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut res = client.request(req).await?;
    println!("Response: {}", res.status());
    if res.status() == 301 {
        if res.headers().contains_key("location") {
            let l = match res.headers().get("location") {
                Some(l) => l,
                _ => res.headers().get("location").unwrap()
            };
            let b64 = format!("Basic {}", base64::encode(format!("{}:{}", username, password)));
            let url = std::str::from_utf8(l.as_bytes()).unwrap().to_string();
            let req = Request::builder()
                .method(Method::GET)
                .uri(url)
                .header("Authorization", b64)
                .body(Body::from(""))?;
    
                res = client.request(req).await?;
        }
    }

    while let Some(chunk) = res.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
