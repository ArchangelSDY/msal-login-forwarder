use config::Config;
use std::collections::HashMap;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ForwardRequest {
    url: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    server: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conf_path = dirs::config_dir().expect("no config dir");
    conf_path.push("msal-login-forwarder");

    let settings = Config::builder()
        .add_source(config::File::with_name(conf_path.to_str().expect("no path")))
        .build()?;
    let settings_content: Settings = settings.try_deserialize()?;

    let argv: Vec<String> = env::args().collect();
    println!("Server: {}, URL: {}", settings_content.server, argv[1]);

    let u = url::Url::parse(&argv[1])?;
    let qs: HashMap<_, _> = u.query_pairs().into_owned().collect();
    let mut ru = url::Url::parse(&qs["redirect_uri"])?;
    let port = ru.port().unwrap();

    let forward_request = ForwardRequest{
        url: argv[1].to_string(),
        port,
    };
    let client = reqwest::blocking::Client::new();
    let resp = client.post(format!("http://{}", settings_content.server))
        .json(&forward_request)
        .send()?;
    let cb_qs = resp.text()?;

    ru.set_query(Some(&cb_qs));
    println!("Callback URL: {}", ru);

    client.get(ru)
        .send()?;

    Ok(())
}

