use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::Path;
use std::str;
use std::time::Instant;

// use ngrammatic::{CorpusBuilder, Pad};

use colored::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    a: String,
    b: String,
    c: Vec<String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct EmojiAll {
//     #[serde(flatten)]
//     inner: HashMap<String, Emoji>,
// }

type EmojiHash = HashMap<String, Emoji>;

fn print_map<K: Debug + Eq + Hash, V: Debug>(map: &HashMap<K, V>) {
    for (k, v) in map.iter() {
        println!("-----");
        println!("{:?}: {:?}", k, v);
        println!("-----");
    }
}

fn main() {
    // Fetch the file
    let json_file_path = Path::new("emojis.json");
    let json_file_str = read_to_string(json_file_path).expect("file not found");
    let start = Instant::now();

    // deserialise to EmojiHash
    let _deserialized_camera: EmojiHash =
        serde_json::from_str(&json_file_str).expect("error while reading json");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let hash_keys = _deserialized_camera.keys();
    // println!("hash_keys: {:?}", hash_keys);
    // println!("emoji json config: {:?}", &_deserialized_camera);

    let key_to_get: &str = "fire";

    // print_map(&_deserialized_camera);
    println!("attempting to find {} in our hashmap", &key_to_get.green());

    let get_the_opp: Option<&Emoji> = _deserialized_camera.get(key_to_get);
    println!(
        "found value where key = {}: {:?}",
        &key_to_get.red(),
        &get_the_opp
    );
}
