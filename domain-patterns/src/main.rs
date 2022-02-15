use std::env;
use viuer::{print_from_file, Config};

use domains::car::{CarFactory, CarType};

pub mod domains;


fn main() {
  let args: Vec<String> = env::args().collect();
  let query = &args[1];
  println!("Getting car type {}", query);

  let car = match &query[..] {
    "bmw" => CarFactory::new_car(&CarType::BmwE30),
    "porche" => CarFactory::new_car(&CarType::Porche911),
    "nissan" => CarFactory::new_car(&CarType::NissanSkylineR34),
    _ => panic!("Invalid input of {}", query),
  };

  car.buy();
  let car_stats = car.get_stats();
  println!("{} as a top speed of {}km/h", car_stats.name, car_stats.top_speed);

  let conf = Config {
    // set offset
    x: 20,
    y: 4,
    // width: Some(24),
    height: Some(24),
    ..Default::default()
  };
  
  // starting from row 4 and column 20,
  // display `img.jpg` with dimensions 80x25 (in terminal cells)
  // note that the actual resolution in the terminal will be 80x50
  print_from_file(car_stats.image, &conf).expect("Image printing failed.");
  
  // let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
  // viuer::print(&img, &conf).expect("Image printing failed.");
}