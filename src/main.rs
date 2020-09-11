extern crate hyper;

use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Jeg æder blåbærsyltetøj!");

    let client = Client::new();
    let uri = "https://www.freebsd.org/".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
