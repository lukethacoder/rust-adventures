use std::{fs, io};

fn main() {
    println!("Running main...");

    for x in 0..10 {
        let file_name: String = format!("./test_folder/{}.txt", x);
        let file_contents: String = format!("This is file number {}", x);
        let creating_file = create_file(file_name.as_str(), file_contents.as_str());
        println!("{}", x); // x: i32
        println!("creating_file: {:?}", creating_file);
    }
    let entries = get_files();
    println!("entries: {:?}", entries);
}

fn create_file(file_name: &str, file_contents: &str) -> std::io::Result<()> {
    // let mut file = File::create(&file_name)?;

    let data = file_contents;
    fs::write(file_name, data).expect("Unable to write file");

    Ok(())
}

fn get_files() -> io::Result<()> {
    let mut entries = fs::read_dir("./test_folder")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    println!("Entries data: {:?}", entries);

    Ok(())
}
