use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str;
use std::time::Instant;

use ngrammatic::{CorpusBuilder, Pad};

use colored::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    a: String,      // Human Readable Name of Emoji
    b: String,      // UTF8 string code
    c: Vec<String>, // Array of string related words
}

type EmojiHash = HashMap<String, Emoji>;

fn main() {
    // Fetch the file
    let json_file_path = Path::new("emojis.json");
    let json_file_str = read_to_string(json_file_path).expect("file not found");

    // Will see how long this query takes
    let start = Instant::now();

    // deserialise to EmojiHash
    let _deserialized_camera: EmojiHash =
        serde_json::from_str(&json_file_str).expect("error while reading json");

    let hash_keys = _deserialized_camera.keys();

    let key_to_get: &str = "fire";
    let mut matched_emojis: Vec<&Emoji> = Vec::new();

    for k in hash_keys {
        let mut corpus = CorpusBuilder::new().arity(2).pad_full(Pad::Auto).finish();

        let _get_the_opp: Option<&Emoji> = _deserialized_camera.get(k);
        let the_emoji = _get_the_opp.unwrap();
        corpus.add_text(k);
        corpus.add_text(&the_emoji.a.to_string());

        for keyword in &the_emoji.c {
            corpus.add_text(&keyword.to_string());
        }

        let results = corpus.search(key_to_get, 0.5);
        let top_match = results.first();

        // Check if we matched one of the values of this emoji obj
        if top_match != None {
            matched_emojis.push(the_emoji);
        }
    }

    let elapsed = start.elapsed();

    println!("--------------------");
    println!(
        "FOUND {} MATCHING EMOJIS for search of '{}' in {:.2?}",
        matched_emojis.len(),
        key_to_get.green(),
        elapsed
    );
    println!("--------------------");
    println!("first result: {:?}", matched_emojis[0]);
}
