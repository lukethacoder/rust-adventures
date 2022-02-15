//! Factory method

pub trait Car {
  fn buy(&self);
  fn get_stats(&self) -> CarProps;
}

pub enum ManufacturerType {
  Bmw,
  Porche,
  Nissan,
  Toyota,
  Mazda,
  Ford,
}

pub struct CarProps {
  pub name: String,
  pub manufacturer: ManufacturerType,
  // Top speed in km/h
  pub top_speed: u64,
  // 0 to 100km time in seconds
  pub zero_to_100: f32,
  pub year: u16,
  // Weight in kgs
  pub weight: u64,
  // Length in mm 
  pub length: u64,
  // Width in mm 
  pub width: u64,
  // Height in mm 
  pub height: u64,
  pub image: String
}

pub enum CarType {
  BmwE30,
  Porche911,
  NissanSkylineR34,
}

struct BmwE30 {}
impl Car for BmwE30 {
  fn buy(&self) {
    println!("buy a BMW E30");
  }
  fn get_stats(&self) -> CarProps {
    return CarProps {
      name: "BWM M3 E30".to_string(),
      manufacturer: ManufacturerType::Bmw,
      top_speed: 235,
      zero_to_100: 6.7,
      year: 1987,
      weight: 1165,
      length: 4345,
      width: 1680,
      height: 1370,
      image: "./bmw.jpg".to_string(),
    }
  }
}

struct Porche911 {}
impl Car for Porche911 {
  fn buy(&self) {
    println!("buy a Porche 911");
  }
  fn get_stats(&self) -> CarProps {
    return CarProps {
      name: "Porsche 911 Turbo 3.3 Coupé".to_string(),
      manufacturer: ManufacturerType::Porche,
      top_speed: 260,
      zero_to_100: 5.0,
      year: 1982,
      weight: 1120,
      length: 4290,
      width: 1700,
      height: 1300,
      image: "./porche.jpg".to_string(),
    }
  }
}

struct NissanSkylineR34 {}
impl Car for NissanSkylineR34 {
  fn buy(&self) {
    println!("buy a Nissan Skyline R34");
  }
  fn get_stats(&self) -> CarProps {
    return CarProps {
      name: "Nissan Skyline GT-R".to_string(),
      manufacturer: ManufacturerType::Nissan,
      top_speed: 260,
      zero_to_100: 4.9,
      year: 2001,
      weight: 1560,
      length: 4600,
      width: 1785,
      height: 1360,
      image: "./skyline.jpg".to_string(),
    }
  }
}

pub struct CarFactory;
impl CarFactory {
  pub fn new_car(s: &CarType) -> Box<dyn Car> {
    match s {
      CarType::Porche911 => Box::new(Porche911 {}),
      CarType::BmwE30 => Box::new(BmwE30 {}),
      CarType::NissanSkylineR34 => Box::new(NissanSkylineR34 {}),
    }
  }
}
