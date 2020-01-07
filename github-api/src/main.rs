#[macro_use]
extern crate serde;
extern crate reqwest;
use reqwest::blocking;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("request_url => {}", request_url);
    let mut response: HashMap<String, String> = reqwest::blocking::get(&request_url)?
    .json()?
    // let mut response = reqwest::blocking::get(&request_url)?;

    // let users: Vec<User> = response.json()?;
    println!("response => {:?}", response);
    Ok(())
}
