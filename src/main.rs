use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    let response_200 = client
        .get("https://httpbin.org/ip")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<GETAPIResponse>()
        .await?;

    println!("{:#?}", response_200);

    // Create a Map of string key-value pairs
    // to represent the body payload
    let mut map = HashMap::new();

    map.insert("lang", "rust");
    map.insert("body", "json");

    // Making a POST Request
    let response_json = client.post("https://httpbin.org/anything").json(&map).send().await?.json::<JSONResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}
