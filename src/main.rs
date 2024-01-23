use reqwest;
use rss::Channel;
use select::document::Document;
use select::predicate::{Attr, Name};

#[tokio::main]
async fn main(){
    let url = "https://phys.org/rss-feed/physics-news/rss/";

    // Build a client with a custom user-agent header
    let client = reqwest::Client::builder()
        .user_agent("Rust Science Feed ")
        .build().unwrap();

    // Perform a GET request to the URL using the custom client
    let response = client.get(url).send().await.unwrap();

    // Read the response body as a string
    let html = response.bytes().await.unwrap();
    let channel = Channel::read_from(&html[..]).unwrap();

    // Print all the titles of the items
    for item in channel.items {
        println!("{:?}", item.title.unwrap());
    }
}