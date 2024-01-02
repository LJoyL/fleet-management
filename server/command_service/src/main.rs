use std::io;
use std::io::{stdin, stdout, Write};

use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Body, Bytes};
use hyper::{body::Buf, Request};
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use tokio::net::TcpStream;

// #[path = "../benches/support/mod.rs"]
// mod support;
// use support::TokioIo;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url: hyper::Uri = "http://localhost:8000/publish".parse().unwrap();
    let mut input = String::new();

    loop {
        println!("Enter s to send request or q to quit");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        // print inputed text
        println!("{}", input);

        if input.starts_with('q') {
            break;
        } else if input.starts_with('s') {
            fetch_json(url.clone()).await?;
        }

        input.clear();
    }

    // print users
    // println!("users: {:#?}", users);

    Ok(())
}

async fn fetch_json(url: hyper::Uri) -> Result<()> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    // Creating body in a json
    let body = r#"{
        "topic": "default",
        "message": "Happy New Year!"
    }"#;

    // Fetch the url...
    let req = Request::post(url)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap();

    let res = sender.send_request(req).await?;

    // Stream the body, writing each frame to stdout as it arrives
    // while let Some(next) = res.frame().await {
    //     let frame = next?;
    //     if let Some(chunk) = frame.data_ref() {
    //         io::stdout().write_all(&chunk).await?;
    //     }
    // }

    Ok(())
}
