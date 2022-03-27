use std::env;
use std::fs::read_to_string;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::from_str;

use colored::*;
use dotenv::dotenv;
use scan_dir::ScanDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let sfdx_project_path = env::var("SFDX_PROJECT_PATH").ok().unwrap();
  println!("Using sfdx_project_path: {:?} ", sfdx_project_path);

  // !!! working
  let all_xml_files: Vec<_> = ScanDir::dirs()
    .walk(sfdx_project_path, |iter| {
      iter
        .filter(|&(ref entry, _)| {
          let path_str = entry.path().into_os_string().into_string().unwrap();

          !path_str.contains("node_modules")
            && path_str.contains("objects")
            && path_str.contains("fields")
        })
        .map(|(ref entry, _)| {
          let all_rs_files: Vec<_> = ScanDir::files()
            .walk(entry.path(), |iter| {
              iter
                .map(|(ref entry, _)| {
                  let path_str = entry.path().into_os_string().into_string().unwrap();
                  let json_file_path = Path::new(&path_str);
                  let json_file_str = read_to_string(json_file_path).expect("file not found");

                  let mut sobject_field: SObjectField = from_str(&json_file_str).unwrap();

                  let path_array: Vec<&str> = path_str.split('\\').collect();
                  sobject_field.sobject = Some(path_array[path_array.len() - 3].to_string());

                  return sobject_field;
                })
                .collect()
            })
            .unwrap();
          return all_rs_files;
        })
        .collect()
    })
    .unwrap();

  let mut field_count = 0;
  for fields in &all_xml_files {
    let first_field = &fields[0];
    println!(
      "{}",
      &first_field.sobject.as_ref().unwrap().to_string().red()
    );
    for field in fields {
      field_count += 1;
      println!(
        "{}.{}",
        &field.sobject.as_ref().unwrap(),
        &field.full_name.as_ref().unwrap()
      )
    }
  }

  println!(
    "\nCollected {} fields from {} objects",
    field_count.to_string().green(),
    &all_xml_files.len().to_string().green()
  );

  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SObjectField {
  pub sobject: Option<String>,
  full_name: Option<String>,
  description: Option<String>,
  external_id: Option<bool>,
  track_feed_history: Option<String>,
  inline_help_text: Option<String>,
  label: Option<String>,
  length: Option<String>,
  visible_lines: Option<String>,
  #[serde(rename = "type")]
  field_type: Option<String>,
}
