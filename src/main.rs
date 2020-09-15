extern crate hyper;

use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Jeg æder blåbærsyltetøj!");

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = "https://freebsd.org/".parse()?;
    let mut res = client.get(uri).await?;
    println!("Response: {}", res.status());
    if res.status() == 301 {
        if res.headers().contains_key("location") {
            let l = match res.headers().get("location") {
                Some(l) => l,
                _ => res.headers().get("location").unwrap()
            };
            let uri = std::str::from_utf8(l.as_bytes()).unwrap().to_string().parse()?;
            res = client.get(uri).await?;
        }
    }

    while let Some(chunk) = res.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
