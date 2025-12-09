use std::{collections::HashMap};
use tokio;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize}; 
use log::{info, error, debug};
use clap::{Parser};
use lazy_static::lazy_static;

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

//HashMap holdin the string-emoji equivalences
//TODO: Include emojis for cards above 10 (Queen, etc.)
lazy_static! {
    static ref CARD_ICONS: HashMap<String, String> = {
        let mut icons = HashMap::new();
        icons.insert("HEARTS".to_string(), "♥️".to_string());
        icons.insert("SPADES".to_string(), "♠️".to_string());
        icons.insert("CLUBS".to_string(), "♣️".to_string());
        icons.insert("DIAMONDS".to_string(), "♦️".to_string());
        icons
    };
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
async fn draw_card(deck_id: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error> > {
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
                        Ok(final_result)
                    },
                    Err(e) => {
                        error!("Error on JSON serialization: {e}");
                        Err("Resource not found (404)".into())
                    }
                }
            },
            StatusCode::NOT_FOUND => {
                error!("URL not found error");
                Err("Resource not found (404)".into())
            },
            status if status.is_client_error() => {
                error!("[ERROR] client error: {:?}", status);
                Err("Client error: (40x)".into())
            },
            status if status.is_server_error() => {
                error!("[ERROR] server side error: {:?}", status);
                Err("Server side error (50x)".into())
            }, _ => {
                error!("[ERROR] Invalid statusCode");
                Err("Unkown status code received".into())
            }
        }
    }
    else{
        //TODO: implement draw_card for a given deck_id
        info!("The string is not empty");
        Ok(final_result)
    }
}

fn main() {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let args = Cli::parse();
    let n_cards = args.n;
    let mut n: u16 = 1;
    if let Some(n_cards) = n_cards {
        n = n_cards;
        info!("N arg not given, drawing only one card");
    }
    for i in 1..=n {
        info!("Requested to draw {} cards", n);
        let future = draw_card("".to_string());
        let final_result = rt.block_on(future);
        match final_result {
            Ok(final_result) => {
                debug!("Ok result evaluated for draw_card {:?}", final_result);
                let  suit = final_result.get("Suit");
                let value = final_result.get("Value");
                if let Some(suit) = suit { 
                    if CARD_ICONS.contains_key(suit) {
                        let icon = CARD_ICONS.get(suit);
                        if let Some(value) = value {
                            println!("Your card number {:?} is {:?} of {:?} ({:?}{:?}) !", i, value, suit, value, icon);
                        }
                    }else{
                        error!("Card suit not defined in CARD_ICONS: {:?}", suit);
                    }
                }
            },
            Err(error) => {
                println!("Error {}", error);
            }
        }
        info!("Card {} out of {} successfully obtained", i, n);
    }
}
