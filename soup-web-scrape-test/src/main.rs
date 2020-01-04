extern crate reqwest;
extern crate soup;

use soup::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://lukesecomb.digital/")?;
    let soup = Soup::from_reader(resp)?;
    let result = soup
        .tag("nav")
        .attr("class", "css-glsk79")
        .find()
        .and_then(|section| section.tag("li").find().map(|a| a.text()));
    println!("result {:?} ", result);
    assert_eq!(result, Some("home.".to_string()));
    Ok(())
}
