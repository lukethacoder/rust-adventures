use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
use toml::Value;

#[derive(Deserialize, Debug)]
struct Config {
    username: String,
    websites: Vec<Website>,
}

#[derive(Deserialize, Debug)]
struct Website {
    name: String,
    url: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    
    // Fetch the file as RAW text data
    let data = read_the_toml_file("config.toml");

    // Serialise the toml file against our defined struct's
    let config: Config = serialise_toml(&data);

    // Say something nice to the user
    println!("\n-------------------------");
    println!("Hi {}. Hope you're ready for a challenge.", config.username);
    println!("------------------------- \n");
    
    println!("-------------------------");
    println!("What should we open");
    println!("-------------------------");
    for website in config.websites {
        println!("{} should open to {}", website.name, website.url);
    }
    println!("-------------------------");

}

fn read_the_toml_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    return contents;
}

fn serialise_toml(data: &str) -> Config {
    // Parse the string of data into toml::Value.
    let v = toml::from_str(data).expect("invalid TOML config format.");

    return v;
}
