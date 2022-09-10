use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {

    let file = File::open("secret.json");
    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut contents = String::new();
    let _result = file.read_to_string(&mut contents);

    println!("{:?}", contents);
    let v: Result<Value> = serde_json::from_str(&contents);
    let v = match v {
        Ok(v) => v,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("{}", v["hello"]);

    api_request().await;

}

async fn api_request(){

    let client = reqwest::Client::new();

    let response = client
    .get("https://api.spotify.com/v1/search")
    .header("AUTHORIZATION", "Bearer [AUTH_TOKEN]")
    .header("CONTENT_TYPE", "application/json")
    .header("ACCEPT", "application/json")
    .send()
    .await
    .unwrap()
    .text()
    .await;
    
    println!("{:?}", response);
    
}