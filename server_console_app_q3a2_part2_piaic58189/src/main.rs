/*
   PIAIC IOT Batch 3 Assignment 2
   Title: server_console_app_q3a2_part2_piaic58189

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

#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn roll_reply(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("PIAIC58189")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    /* Service to handle incoming connection requests. */
    let make_svc = make_service_fn(|_conn| {
        /* This is the `Service` that will handle the connection.
           `service_fn` is a helper to convert a function that
           returns a Response into a `Service`.*/
        async { Ok::<_, Infallible>(service_fn(roll_reply)) }
    });

    let addr = ([127, 0, 0, 1], 58189).into();

    /* Bind server address+port to service which will handle requests. */
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
