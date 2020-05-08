use std::fs;

use colored::*;
use serde::Deserialize;
use webbrowser;

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

fn main() {
    // Fetch the file as RAW text data
    let data = read_the_toml_file("config.toml");

    // Serialise the toml file against our defined struct's
    let config: Config = serialise_toml(&data);

    // Say something nice to the user
    println!("\n--------------------------------------------------");
    println!(
        "Hi {}. Hope you're ready for a challenge.",
        config.username.blue()
    );
    println!("-------------------------------------------------- \n");

    println!("--------------------------------------------------");
    println!("Opening websites to get your day going.");
    println!("-------------------------------------------------- \n");
    for website in config.websites {
        if webbrowser::open(&website.url).is_ok() {
            println!(
                "Successfully opened {} in your default browser.\n",
                &website.name.green()
            );
        } else {
            println!(
                "Failed to open {} in your default browser.\n",
                &website.name.red()
            );
        }
    }
    println!("--------------------------------------------------");
}

fn read_the_toml_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    return contents;
}

fn serialise_toml(data: &str) -> Config {
    // Parse the string of data into toml::Value.
    let v = toml::from_str(data).expect("invalid TOML config format.");

    return v;
}
