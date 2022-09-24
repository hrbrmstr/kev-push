use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Result, Error};

use pushover::API;
use pushover::requests::message::SendMessage;

use serde_derive::{Serialize, Deserialize};

const KEV_JSON_URL: &str = "https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json";
const KEV_CATALOG_URL: &str = "https://www.cisa.gov/known-exploited-vulnerabilities-catalog";

/// This enables (de)serialization of the KEV JSON as of 2022-09-024
#[derive(Debug, Serialize, Deserialize)]
pub struct Kev {
	#[serde(rename = "title")]
	pub(crate) title: String,
	
	#[serde(rename = "catalogVersion")]
	pub(crate) catalog_version: Option<String>,
	
	#[serde(rename = "dateReleased")]
	pub(crate) date_released: String,
	
	#[serde(rename = "count")]
	pub(crate) count: Option<i64>,
	
	#[serde(rename = "vulnerabilities")]
	pub(crate) vulnerabilities: Option<Vec<Vulnerability>>,
}

/// This enables (de)serialization of the KEV JSON as of 2022-09-024
#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
	#[serde(rename = "cveID")]
	pub(crate) cve_id: String,
	
	#[serde(rename = "vendorProject")]
	pub(crate) vendor_project: String,
	
	#[serde(rename = "product")]
	pub(crate) product: String,
	
	#[serde(rename = "vulnerabilityName")]
	pub(crate) vulnerability_name: String,
	
	#[serde(rename = "dateAdded")]
	pub(crate) date_added: String,
	
	#[serde(rename = "shortDescription")]
	pub(crate) short_description: String,
	
	#[serde(rename = "requiredAction")]
	pub(crate) required_action: String,
	
	#[serde(rename = "dueDate")]
	pub(crate) due_date: String,
	
	#[serde(rename = "notes")]
	pub(crate) notes: String,
}

/// This is used to read the locally cached KEV JSON file
pub fn read_kev_cache_from_file<P: AsRef<Path>>(path: P) -> Result<Kev, Error> {

	let file = File::open(path)?;
	let reader = BufReader::new(file);
	
	let u = serde_json::from_reader(reader)?;

	Ok(u)
}

/// This uses Pushover to notify when there's a new KEV release
pub fn notify() {

  if let Ok(token) = env::var("PUSHOVER_APP") {
		if let Ok(user_key) = env::var("PUSHOVER_USER") {

			let api = API::new();

			let msg = SendMessage::new(token, user_key, format!("New KEV Update! {}", KEV_CATALOG_URL));

			if let Err(response) = api.send(&msg) {
			  eprintln!("{:?}", response)
			}

		} else {
			eprintln!("PUSHOVER_USER environment variable is not set!")
		}
	} else {
		eprintln!("PUSHOVER_APP environment variable is not set!")
	}

}

/// This is just a wrapper which makes it clearer what we're creating.
pub fn create_kev_cache_file<P: AsRef<Path>>(kev_cache_file_path: P) -> Result<File, std::io::Error> {
	File::create(kev_cache_file_path)
}

/// This is just a wrapper function which makes it clearer what we're fetching.
pub fn read_kev_from_cisa() -> Result<Kev, reqwest::Error> {
	reqwest::blocking::get(KEV_JSON_URL)?.json::<Kev>()
}