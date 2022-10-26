lazy_static! {
    pub static ref SETTINGS: RwLock<Setting> = RwLock::new(Setting::default());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    theme: u8,
    lang: String,
    exclude_index_path: Vec<String>,
    ext: HashMap<String, String>,
}
