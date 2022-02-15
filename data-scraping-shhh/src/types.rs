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
  id: i64,
  status: Status,
  property_class: PropertyClass,
  listing_type: ListingType,
  created: String,
  updated: String,
  #[serde(rename = "priceView")]
  price_view: String,
  address_postcode: String,
  address_suburb: String,
  address_suburb_display: bool,
  address_state: State,
  address_region: String,
  location: Option<Location>,
  headline: String,
  bedrooms: i64,
  bathrooms: i64,
  parking: i64,
  price: Option<i64>,
  price_display: bool,
  #[serde(rename = "underOffer")]
  under_offer: bool,
  authority: Authority,
  auction_date: Option<String>,
  #[serde(rename = "auctionVenue")]
  auction_venue: Option<String>,
  rent: Option<serde_json::Value>,
  rent_period: RentPeriod,
  #[serde(rename = "dateAvailable")]
  date_available: Option<String>,
  #[serde(rename = "commercialRent")]
  commercial_rent: i64,
  #[serde(rename = "commercialRent_period")]
  commercial_rent_period: CommercialRentPeriod,
  #[serde(rename = "carSpaces")]
  car_spaces: i64,
  #[serde(rename = "parkingComments")]
  parking_comments: String,
  area: Option<String>,
  area_unit: AreaUnit,
  address_display_string: String,
  address_street_string: String,
  images: Vec<ImageElement>,
  agents: Option<serde_json::Value>,
  office: Office,
  url_path: String,
  category: String,
  slug: String,
  inspections: Vec<Inspection>,
  #[serde(rename = "buildingDetails_energyRating")]
  building_details_energy_rating: Option<String>,
  last_price_change: Option<serde_json::Value>,
  development: Option<serde_json::Value>,
  #[serde(rename = "soldDetails_price")]
  sold_details_price: Option<serde_json::Value>,
  #[serde(rename = "soldDetails_date")]
  sold_details_date: Option<serde_json::Value>,
  #[serde(rename = "leasedDetails_price")]
  leased_details_price: Option<serde_json::Value>,
  stamp_duty: Option<String>,
  development_show_full_result: bool,
  has_live_auction: bool,
  has_virtual_tour: bool,
  first_go_live: String,
  has_new_price: bool,
  body_corporate_fees: String,
  suburb_aliases: Option<Vec<String>>,
  is_private_listing: Option<bool>,
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
  #[serde(rename = "type")]
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
  #[serde(rename = "show_1form")]
  show_1_form: bool,
  #[serde(rename = "show_1form_url")]
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
  #[serde(rename = "ACT", alias = "Australian Capital Territory")]
  Act,
  #[serde(rename = "NSW", alias = "New South Wales")]
  Nsw,
  #[serde(rename = "QLD", alias = "Queensland")]
  Qld,
  #[serde(rename = "VIC", alias = "Victoria")]
  Vic,
  #[serde(rename = "SA", alias = "South Australia")]
  Sa,
  #[serde(rename = "WA", alias = "Western Australia")]
  Wa,
  #[serde(rename = "NT", alias = "Northern Territory")]
  Nt,
  #[serde(rename = "TAS", alias = "Tasmania")]
  Tas,
  #[serde(rename = "")]
  Empty,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AreaUnit {
  #[serde(rename = "acre")]
  Acre,
  #[serde(rename = "hectare")]
  Hectare,
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "square")]
  Square,
  #[serde(rename = "squareMeter")]
  SquareMeter,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Authority {
  #[serde(rename = "auction")]
  Auction,
  #[serde(rename = "exclusive")]
  Exclusive,
  #[serde(rename = "sale")]
  Sale,
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "conjunctional")]
  Conjunctional,
  #[serde(rename = "multilist")]
  Multilist,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CommercialRentPeriod {
  #[serde(rename = "annual")]
  Annual,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ListingType {
  #[serde(rename = "lease")]
  Lease,
  #[serde(rename = "sale")]
  Sale,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Type {
  Point,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CalculatorProviderName {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "Loan Market Horizon")]
  LoanMarketHorizon,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PropertyClass {
  #[serde(rename = "commercial")]
  Commercial,
  #[serde(rename = "land")]
  Land,
  #[serde(rename = "rental")]
  Rental,
  #[serde(rename = "residential")]
  Residential,
  #[serde(rename = "holidayRental")]
  HolidayRental,
  #[serde(rename = "rural")]
  Rural,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RentPeriod {
  #[serde(rename = "week")]
  Week,
  #[serde(rename = "weekly")]
  Weekly,
  #[serde(rename = "monthly")]
  Monthly,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
  #[serde(rename = "current")]
  Current,
}
