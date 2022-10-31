use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Client which is re-used between requests
    let client = reqwest::Client::new();

    // Making a GET Request
    let response_200 = client.get("https://httpbin.org/ip").header(CONTENT_TYPE, "application/json").send().await?.json::<GETAPIResponse>().await?;

    println!("{:#?}", response_200);

    Ok(())
}
