use std::collections::HashMap;

//use hyper::{Client, Body, Request};
//use hyper::client::HttpConnector;
//use hyper::body::HttpBody;
use tokio::runtime::Runtime;
use tokio;
use reqwest::{Client, Error, StatusCode};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, KeyValueMap};
use serde_json::to_string; //Result;

const BASE_URL: &str = "https://www.deckofcardsapi.com/aapi/deck/";
const NEW_DECK_ENDPOINT: &str = "new/shuffle/?deck_count=1";
const DRAW_CARD_ENDPOINT: &str = "/draw/?count=1";

async fn get_request() -> Result<(), Error>  {
    let rest_client = reqwest::Client::new();
    let response = rest_client.get(BASE_URL).send().await?;
    println!("Status: {:?}", response.text().await?);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    pub success: bool,
    pub deck_id: String,
    pub remaining: i16,
    pub shuffled: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct Images {
    svg: String,
    png: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Card {
    code: String,
    image: String,
   // #[serde_as(as "KeyValueMap<Images>")]
    images: HashMap<String, String>,
    value: String,
    suit: String
}

#[derive(Debug, Serialize, Deserialize)]
struct CardResponse {
    success: bool,
    deck_id: String,
    cards: [Card; 1],
    remaining: i16
}

async fn draw_card(deck_id: String) -> Result<(), Error> {
    println!("On drawCard fn with deck_id: {:?}", deck_id.is_empty());
    if deck_id.is_empty() {
        //A deck_id was not provided, get a new deck
        let request_url = BASE_URL.to_string()+"new"+DRAW_CARD_ENDPOINT; 
        println!("[INFO] Requesting new deck of cards");
        let client = Client::new();
        let response = client.get(request_url)
        .send()
        .await?;
        //.text()
        //.await?;

        //if let Err(e) = response {
        //    println!("Error {:}, e");
        //}
        //
        println!("[INFO] response status: {:?} ", &response.status());
        let status = &response.status();
        match response.status() {
            StatusCode::OK => {
                println!("[INFO] status code OK: {:?}", status);
            },
            StatusCode::NOT_FOUND => {
                println!("[ERROR] url not found: {:?}", status);
            }
            _ => {
                println!("[ERROR] Invalid statusCode");
            }
        }

        let response_txt = response.text().await?;
        //let body: Card = response.json().await?;
        println!("[INFO] Client response: {:?}", &response_txt);
        //.expect("[ERROR] Failed to get payload")
        //.text()
        let body: CardResponse  = serde_json::from_str(response_txt.as_str()).unwrap(); 
        //.json()::<Body>()
        //.await?;
        //intln!("[INFO] Response body: {:?}", body);
        println!("[INFO] Body: {:?}", body.remaining);
    }
    else{
        println!("The string is not empty");
    }
    println!("Finish draw_card");
    Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = draw_card("".to_string());//response.deck_id);
    let _ = rt.block_on(future);
    println!("---");
}

/*
#[tokio::main]
async fn main()-> Result<(), Error> {
    let newCard: Card = Card{ctype: "Hearts".to_string(), number: 11, color: "Red".to_string()};
    let newCardStr = to_string(&newCard);
    if newCardStr.is_ok() {
        println!("Newlly created card: {:?}", newCardStr.ok().unwrap());
    }
    else {
        println!("{:?}", newCardStr);
    }
    let concat = BASE_URL.to_string() + NEW_DECK_ENDPOINT;
    println!(">{:?}", concat);
    //let response: Body = reqwest::get(BASE_URL.to_string()+NEW_DECK_ENDPOINT)
    let response: Body = Client::new().get(BASE_URL.to_string()+NEW_DECK_ENDPOINT)
        .send()
        .await
        .expect("failed to get payload")
        .json()
        //.text()
        //.json()::<Body>()
        .await?;
    println!("Status: {:?}", response);
    println!("---");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = draw_card("".to_string());//response.deck_id);
    let _ = rt.block_on(future);
    println!("---");
    //get_request();
    Ok(())
    //get_request();
    /*
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = Client::new();
    });
    */
}
*/
