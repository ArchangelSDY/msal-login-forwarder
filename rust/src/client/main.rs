use std::collections::HashMap;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ForwardRequest {
    url: String,
    port: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();
    println!("URL: {}", argv[1]);

    let u = url::Url::parse(&argv[1])?;
    let qs: HashMap<_, _> = u.query_pairs().into_owned().collect();
    let mut ru = url::Url::parse(&qs["redirect_uri"])?;
    let port = ru.port().unwrap();

    let forward_request = ForwardRequest{
        url: argv[1].to_string(),
        port,
    };
    let client = reqwest::blocking::Client::new();
    let resp = client.post("http://192.168.98.1:9080")
        .json(&forward_request)
        .send()?;
    let cb_qs = resp.text()?;

    ru.set_query(Some(&cb_qs));
    println!("Callback URL: {}", ru);

    client.get(ru)
        .send()?;

    Ok(())
}

