//use hyper::{Client, Body, Request};
//use hyper::client::HttpConnector;
//use hyper::body::HttpBody;
use tokio::runtime::Runtime;
use tokio;
use reqwest::{Client, Error};
use serde::{Deserialize};
use serde_json::Result;

const BASE_URL: &str = "https://www.deckofcardsapi.com/api/deck/";
const NEW_DECK_ENDPOINT: &str = "new/shuffle/?deck_count=1";

async fn get_request() -> Result<(), Error>  {
    let rest_client = reqwest::Client::new();
    let response = rest_client.get(BASE_URL).send().await?;
    println!("Status: {:?}", response.text().await?);
    Ok(())
}

#[derive(Deserialize)]
struct Body {
    success: bool,
    deck_id: String,
    remaining: i16,
    shufled: bool
}

#[tokio::main]
async fn main() {
    let concat = BASE_URL.to_string() + NEW_DECK_ENDPOINT;
    println!(">{:?}", concat);
    let response = reqwest::get(BASE_URL.to_string()+NEW_DECK_ENDPOINT)
        .await
        .expect("failed to get payload")
        .text()
        //.json()::<Body>()
        .await;
    println!("Status: {:?}", response);
    println!("Hello, world!");
    //get_request();
    /*
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = Client::new();
    });
    */
}
