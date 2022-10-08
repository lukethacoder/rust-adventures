# audio-search

Two methods can be run:

1. First to fetch audio data based on a folder provided
2. Run the search on the cached audio data (saved into a json file)

each should be toggled as you want/need the functionality

```rust
fn main() -> tantivy::Result<()> {
    // toggle between true/false
    if false {
        // Fetch audio data and save to the local JSON file
        walk(&norm(BASE_AUDIO_DIRECTORY).to_string());
    }

    // toggle between true/false
    if false {
        search();
    }

    Ok(())
}
```
