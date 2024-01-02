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

        if input.starts_with('q') {
            // exit
            break;
        } else if input.starts_with('s') {
            if input.len() > 1 {
                let mut input_split = input.split_whitespace();
                let _ = input_split.next();
                let message = input_split.next().unwrap_or("");
                let delay = input_split.next().unwrap_or("");
                let delay = delay.parse::<i32>().unwrap_or(0);

                // send request
                send_request_command(url.clone(), String::from(message), delay).await?;
            }
            // send request
            else {
                send_request_command(url.clone(), String::from("Happy New Year!"), 0).await?;
            }
        }

        input.clear();
    }

    // print users
    // println!("users: {:#?}", users);

    Ok(())
}

async fn send_request_command(url: hyper::Uri, message: String, delay: i32) -> Result<()> {
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

    // for encryped connection
    // let authority = url.authority().unwrap().clone();

    // Creating body in a json
    let mut body = format!(
        r#"{{"topic": "default", 
            "message": {{ 
                "text": "{}"
            }}
        }}"#,
        message
    );

    if delay > 0 {
        body = format!(
            r#"{{"topic": "default", 
                "message": {{ 
                    "text": "{}",
                    "delay": {}
                }}
            }}"#,
            message, delay
        );
    }

    let req = Request::post(url)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap();

    let _res = sender.send_request(req).await?;

    // Todo: handle response

    Ok(())
}
