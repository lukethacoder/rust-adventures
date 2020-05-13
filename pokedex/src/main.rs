use serde::Deserialize;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct NameUrl {
  name: String,
  url: String,
}

#[derive(Deserialize, Debug)]
struct Abilities {
  ability: NameUrl,
  is_hidden: bool,
  slot: i32,
}

#[derive(Deserialize, Debug)]
struct Stats {
  base_stat: i32,
  effort: i32,
  stat: NameUrl,
}

#[derive(Deserialize, Debug)]
struct Pokemon {
  id: i32,
  name: String,
  height: i32,
  order: i32,
  stats: Vec<Stats>,
  abilities: Vec<Abilities>,
}

impl Pokemon {
  fn basic_response(&self) {
    println!("┌──────────────────────────────────────────────────┐");
    println!("| Pokemon: {}", self.name);
    println!("├──────────────────────────────────────────────────┤");
    println!("| Height: {}dm", self.height);
    println!("| Order: {}", self.order);
    println!("| Number of Abilities: {}", self.abilities.len());
    println!("| Number of Stats: {}", self.stats.len());
    println!("└──────────────────────────────────────────────────┘");
  }
}

fn response_to_pokemon(value: serde_json::Value) -> Pokemon {
  serde_json::from_value(value).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  println!("Enter a pokemon name");

  let s = get_user_input();
  println!("Searching for pokemon: {}", &s);
  let request_url = format!(
    "https://pokeapi.co/api/v2/pokemon/{user_query}",
    user_query = &s
  );

  let echo_json: serde_json::Value = reqwest::Client::new()
    .get(&request_url)
    .send()
    .await?
    .json()
    .await?;

  // println!("{:#?}", &echo_json);
  let pokemon = response_to_pokemon(echo_json);

  pokemon.basic_response();

  Ok(())
}

fn get_user_input() -> String {
  let mut s = String::new();
  let _ = stdout().flush();
  stdin()
    .read_line(&mut s)
    .expect("Did not enter a correct string");
  if let Some('\n') = s.chars().next_back() {
    s.pop();
  }
  if let Some('\r') = s.chars().next_back() {
    s.pop();
  }
  return s;
}
