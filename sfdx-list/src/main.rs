use std::{env,fs};
use colored::*;
use regex::Regex;

mod schema;

const PATH_VAR: &str = "HOMEPATH";
const STATE_FOLDER: &str = ".sfdx";

fn main() {
  println!("Running main...\n");
  // regex to check for valid json files
  // "borrowed" from https://github.com/forcedotcom/sfdx-core/blob/552d6f2301b29f03ca9cb0cdc293c86a0e281aed/src/authInfo.ts#L319
  let auth_file_name_regex = Regex::new(r"^[^.][^@]*@[^.]+(\.[^.\s]+)+\.json$").unwrap();

  let home_path = env::var(PATH_VAR).ok();
  let path_to_sfdx: &str = &format!("{}\\{}", home_path.unwrap(), STATE_FOLDER);
  println!("{} files to check\n", path_to_sfdx.len());

  let mut list_of_orgs: Vec<schema::Org> = vec![];

  for entry_res in fs::read_dir(path_to_sfdx).unwrap() {
    let entry = entry_res.unwrap();
    let file_name_buf = entry.file_name();
    let file_name = file_name_buf.to_str().unwrap();
    
    // check valid auth file
    if auth_file_name_regex.is_match(&file_name) {
      let path_to_file = format!("{}\\{}", path_to_sfdx, file_name);
      let file_contents_as_string: String = get_file(&path_to_file);
          
      // Parse the string of data into a Person object. This is exactly the
      // same function as the one that produced serde_json::Value above, but
      // now we are asking it for a Person as output.
      let org_data: schema::Org = serde_json::from_str(file_contents_as_string.as_str()).unwrap();
      println!("{} {}", org_data.username.blue(), org_data.org_id);
      list_of_orgs.push(org_data);
    }
  }
  
  println!("\nFound {} orgs", list_of_orgs.len());
}

fn get_file(file_path: &str) -> String {
  let contents = fs::read_to_string(file_path)
    .expect("Something went wrong reading the file");
  return contents;
}