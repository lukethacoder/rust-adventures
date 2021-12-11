use std::process::Command;
use colored::*;

#[async_std::main]
async fn main() {
  println!("Running main...\n");
  main_async().await;
  print!("{} ", "Success".green());
}

async fn main_async() {
  let org_alias = "luke-dev";
  println!("Opening Salesforce Instance {} ", org_alias.blue());

  if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args([
        "/C",
        format!("sf env open -e {}", org_alias.to_string()).as_str(),
      ])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(format!("sf env open -e {}", org_alias.to_string()).as_str())
      .output()
      .expect("failed to execute process")
  };
}