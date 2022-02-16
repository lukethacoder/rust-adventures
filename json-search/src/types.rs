use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseObject {
  pub count: i64,
  pub results: Vec<Result>,
  pub links: Links,
  pub search_description: SearchDescription,
  pub listing_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Links {
  pub next: Option<serde_json::Value>,
  pub previous: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Result {
  pub id: i64,
  pub status: Status,
  pub property_class: PropertyClass,
  pub listing_type: ListingType,
  pub created: String,
  pub updated: String,
  pub price_view: String,
  pub address_postcode: String,
  pub address_suburb: String,
  pub address_suburb_display: bool,
  pub address_state: State,
  pub address_region: String,
  pub location: Option<Location>,
  pub headline: String,
  pub bedrooms: i64,
  pub bathrooms: i64,
  pub parking: i64,
  pub price: Option<i64>,
  pub price_display: bool,
  pub under_offer: bool,
  pub authority: Authority,
  pub auction_date: Option<String>,
  pub auction_venue: Option<String>,
  pub rent: Option<serde_json::Value>,
  pub rent_period: RentPeriod,
  pub date_available: Option<String>,
  pub commercial_rent: i64,
  pub commercial_rent_period: CommercialRentPeriod,
  pub car_spaces: i64,
  pub parking_comments: String,
  pub area: Option<String>,
  pub area_unit: AreaUnit,
  pub address_display_string: String,
  pub address_street_string: String,
  pub images: Vec<ImageElement>,
  pub agents: Option<serde_json::Value>,
  pub office: Office,
  pub url_path: String,
  pub category: String,
  pub slug: String,
  pub inspections: Vec<Inspection>,
  pub building_details_energy_rating: Option<String>,
  pub last_price_change: Option<serde_json::Value>,
  pub development: Option<serde_json::Value>,
  pub sold_details_price: Option<serde_json::Value>,
  pub sold_details_date: Option<serde_json::Value>,
  pub leased_details_price: Option<serde_json::Value>,
  pub stamp_duty: Option<String>,
  pub development_show_full_result: bool,
  pub has_live_auction: bool,
  pub has_virtual_tour: bool,
  pub first_go_live: String,
  pub has_new_price: bool,
  pub body_corporate_fees: String,
  pub suburb_aliases: Option<Vec<String>>,
  pub is_private_listing: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageElement {
  image: ImageImage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageImage {
  image_800_600: String,
  image_1200_680: String,
  image_320_240: String,
  image_480_270: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inspection {
  inspection_time: String,
  inspection_end_time: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
  location_type: Type,
  coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Office {
  title: String,
  office_name: String,
  logo: String,
  logo_medium: String,
  branding_colour: String,
  branding_font_colour: String,
  slug: String,
  address: String,
  suburb: String,
  postcode: String,
  state: State,
  id: i64,
  show_1_form: bool,
  show_1_form_url: Option<serde_json::Value>,
  sorted_services_id: String,
  inspect_real_estate_id: String,
  inspect_real_estate_sale_id: String,
  inspect_real_estate_apply_id: String,
  snug_office_id: String,
  snug_apply_now_enabled: bool,
  snug_book_inspections_enabled_sale: bool,
  snug_book_inspections_enabled_rent: bool,
  payment_terms: i64,
  calculator_provider_name: CalculatorProviderName,
  calculator_provider_image: Option<String>,
  calculator_provider_url: String,
  phone: String,
  live: bool,
  website_url: String,
  facebook_url: String,
  linkedin_url: String,
  instagram_url: String,
  twitter_url: String,
  youtube_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchDescription {
  search: String,
  locations: String,
  sold_leased_search: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum State {
  Act,
  Nsw,
  Qld,
  Vic,
  Sa,
  Wa,
  Nt,
  Tas,
  Empty,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AreaUnit {
  Acre,
  Hectare,
  Empty,
  Square,
  SquareMeter,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Authority {
  Auction,
  Exclusive,
  Sale,
  Open,
  Conjunctional,
  Multilist,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CommercialRentPeriod {
  Annual,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ListingType {
  Lease,
  Sale,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Type {
  Point,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CalculatorProviderName {
  Empty,
  LoanMarketHorizon,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PropertyClass {
  Commercial,
  Land,
  Rental,
  Residential,
  HolidayRental,
  Rural,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RentPeriod {
  Week,
  Weekly,
  Monthly,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
  Current,
}
