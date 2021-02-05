
/*
   PIAIC IOT Batch 3 Assignment 2
   Title: client_console_app_q3a2_part1_piaic58189

   Create a client-server console application using rust h2 crate

  (Crates.io link: https://crates.io/crates/h2,
  Git: https://github.com/hyperium/h2). 

  Implement the following functionality.
  ======================================

  a. The client sends a message to the server
  b. The server receives a message from the client and sends a
  c. response to the message back to the client
  d. Client gets a response from the server and print it on the console.


   @Author Muhammad Osaid PIAIC58189 
*/


use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let url = "http://127.0.0.1:58189".parse::<hyper::Uri>().unwrap();

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }   
    Ok(())
}