use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use notify::DebouncedEvent::{Create, Remove, Rename, Write};
use notify::{watcher, ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use walkdir::WalkDir;

use crate::store::LUKES_STORE;

mod store;

#[macro_use]
extern crate lazy_static;

const FOLDER_TO_WATCH: &'static str = "./watchin/";

fn log_store() {
    let store = LUKES_STORE.read().unwrap();
    println!("\nstore");
    println!("  paths {:?} ", store.paths);
    println!("  creates {:?} ", store.creates.len());
    println!("  removes {:?} ", store.removes.len());
    println!("  renames {:?} ", store.renames.len());
    println!("  writes {:?} ", store.writes.len());
}

pub fn norm(path: &str) -> String {
    str::replace(path, "\\", "/")
}

fn main() {
    println!(
        "{} total existing files",
        WalkDir::new(FOLDER_TO_WATCH.to_string())
            .into_iter()
            .count()
    );

    // do_run()
    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    let mut watcher: ReadDirectoryChangesWatcher = watcher(sender, Duration::from_secs(1)).unwrap();

    watcher
        .watch(FOLDER_TO_WATCH.to_string(), RecursiveMode::Recursive)
        .unwrap();

    // Spawn off a new thread for handling changes
    thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(event) => {
                    match &event {
                        Create(path_buff) => {
                            LUKES_STORE
                                .write()
                                .unwrap()
                                .create(norm(path_buff.to_str().unwrap()));
                            log_store()
                        }
                        Remove(path_buff) => {
                            LUKES_STORE
                                .write()
                                .unwrap()
                                .remove(norm(path_buff.to_str().unwrap()));
                            log_store()
                        }
                        Rename(source, destination) => {
                            LUKES_STORE.write().unwrap().rename(
                                norm(source.to_str().unwrap()),
                                norm(destination.to_str().unwrap()),
                            );
                            log_store()
                        }
                        Write(path_buff) => {
                            LUKES_STORE
                                .write()
                                .unwrap()
                                .write(norm(path_buff.to_str().unwrap()));
                            log_store()
                        }
                        _ => println!("other event? {:?}", event), // don't care about other types
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    // keep things going on this thread
    loop {}
}
