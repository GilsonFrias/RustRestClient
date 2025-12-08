use std::{collections::HashMap};
use tokio;
use reqwest::{Client, Error, StatusCode};
use serde::{Deserialize, Serialize}; 
use log::{info, error, debug};
use clap::{Parser};

//URL constants defining the request paths
const BASE_URL: &str = "https://www.deckofcardsapi.com/api/deck/";
const DRAW_CARD_ENDPOINT: &str = "/draw/?count=1";

//Struct to format single card entity from /draw response 
#[derive(Debug, Serialize, Deserialize)]
struct CardResponse {
    success: bool,
    deck_id: String,
    cards: [Card; 1],
    remaining: i16
}

//Card node representation struct
#[derive(Debug, Serialize, Deserialize)]
struct Card {
    code: String,
    image: String,
    images: HashMap<String, String>,
    value: String,
    suit: String
}

#[derive(Parser)]
struct Cli {
    //TODO: create --help arg
    n: Option<u16>
}

/*
Function to request a single card draw from remote /draw entity.
input args: deck_id the deck from which a new card will be retrieved
output args: CardResponse struct, Error entity

TODO: Enable 'count' param as input arg to request more than 1 card at a time
*/
async fn draw_card(deck_id: String) -> Result<HashMap<String, String>, Error> {
    //Request a new deck if no deck_id was provided
    let mut final_result = HashMap::new();
    if deck_id.is_empty() {
        let request_url = BASE_URL.to_string()+"new"+DRAW_CARD_ENDPOINT; 
        info!("Requesting new deck of cards ({request_url})");
        let client = Client::new();
        let response = client.get(request_url)
        .send() 
        .await?;
        let status = &response.status();
        info!("Processing response with status: {status}");
        match response.status() {
            StatusCode::OK => {
                info!("Handling success response, serializing into JSON");
                let response_txt = response.text().await?;
                //TODO: implement text extraction error handling
                debug!("Deserialized response: {response_txt}");
                let body: Result<CardResponse, serde_json::Error>  = serde_json::from_str(response_txt.as_str());//.unwrap();
                match body {
                    Ok(data) => {
                        debug!("Successful JSON serialization: {:?}", data);
                        debug!("Suit: {:?}", data.cards[0].suit);
                        final_result.insert("Suit".to_string(), data.cards[0].suit.clone());
                        final_result.insert("Value".to_string(), data.cards[0].value.clone());
                        debug!("Return Hashmap: {:?}", final_result);
                    },
                    Err(e) => {
                        error!("Error on JSON serialization: {e}");
                    }
                }
            },
            StatusCode::NOT_FOUND => {
                error!("URL not found error");
            },
            status if status.is_client_error() => {
                error!("[ERROR] client error: {:?}", status);
            },
            status if status.is_server_error() => {
                error!("[ERROR] server side error: {:?}", status);
            }            _ => {
                error!("[ERROR] Invalid statusCode");
            }
        }
    }
    else{
        //TODO: implement draw_card for a given deck_id
        info!("The string is not empty");
    }
    Ok(final_result)
}

fn main() {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let args = Cli::parse();
    println!("♥️");
    let n_cards = args.n;
    let mut n: u16 = 1;
    if let Some(n_cards) = n_cards {
        n = n_cards;
        info!("N arg not given, drawing only one card");
    }
    println!("n is: {}", n);
    for i in 1..=n {
        info!("Requested to draw {} cards", n);
        let future = draw_card("".to_string());
        let final_result = rt.block_on(future);
        info!("Card {} out of {} successfully obtained", i, n);
    }
    /*
    }else {
        info!("N arg not given, drawing only one card");
    };
    */
    //println!("pattern: {:?}, path: {:?}", pattern, path);
}
