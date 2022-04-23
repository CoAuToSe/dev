// // fn main() {
// //     println!("Hello, world!");
// // }
// use std::convert::Infallible;
// use std::net::SocketAddr;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

// async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello, World".into()))
// }

// #[tokio::main]
// async fn main() {
//     // We'll bind to 127.0.0.1:3000
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     // A `Service` is needed for every connection, so this
//     // creates one from our `hello_world` function.
//     let make_svc = make_service_fn(|_conn| async {
//         // service_fn converts our function into a `Service`
//         Ok::<_, Infallible>(service_fn(hello_world))
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     // Run this server for... forever!
//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }

// // #![deny(warnings)]

// // use std::convert::Infallible;

// // use hyper::service::{make_service_fn, service_fn};
// // use hyper::{Body, Request, Response, Server};

// // async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
// //     Ok(Response::new(Body::from("Hello World!")))
// // }

// // #[tokio::main]
// // pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
// //     pretty_env_logger::init();

// //     // For every connection, we must make a `Service` to handle all
// //     // incoming HTTP requests on said connection.
// //     let make_svc = make_service_fn(|_conn| {
// //         // This is the `Service` that will handle the connection.
// //         // `service_fn` is a helper to convert a function that
// //         // returns a Response into a `Service`.
// //         async { Ok::<_, Infallible>(service_fn(hello)) }
// //     });

// //     let addr = ([127, 0, 0, 1], 3000).into();

// //     let server = Server::bind(&addr).serve(make_svc);

// //     println!("Listening on http://{}", addr);

// //     server.await?;

// //     Ok(())
// // }

#![deny(warnings)]
#![warn(rust_2018_idioms)]
use std::env;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}