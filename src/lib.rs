//! # kev-push
//!
//! This crate builds a binary that will compare
//! [CISA's current KEV Catalog](https://www.cisa.gov/known-exploited-vulnerabilities-catalog";)
//! to a locally cached copy and send a [Pushover](https://pushover.net) notification
//! if there is a new update.
//!
//! You can, say, put it in a cron job to check at some regularity and be notified whenever
//! there is a new addition to the catalog.

use std::env;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Result, Error};

use platform_dirs::AppDirs;

use pushover::API;
use pushover::requests::message::SendMessage;

use serde_derive::{Serialize, Deserialize};

#[cfg(target_os="macos")]
use mac_notification_sys::*;

const KEV_JSON_URL: &str = "https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json";
const KEV_CATALOG_URL: &str = "https://www.cisa.gov/known-exploited-vulnerabilities-catalog";

/// This struct enables (de)serialization of the KEV JSON as of 2022-09-24
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

/// This struct enables (de)serialization of the KEV JSON as of 2022-09-24
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

	let kev = serde_json::from_reader(reader)?;

	Ok(kev)
}

/// This is just a wrapper which makes it clearer what we're creating.
pub fn create_kev_cache_file<P: AsRef<Path>>(kev_cache_file_path: P) -> Result<File, std::io::Error> {
	File::create(kev_cache_file_path)
}

/// This is just a wrapper function which makes it clearer what we're fetching.
pub fn read_kev_from_cisa() -> Result<Kev, reqwest::Error> {
	reqwest::blocking::get(KEV_JSON_URL)?.json::<Kev>()
}

/// This uses Pushover to notify when there's a new KEV release
/// You need to have the `PUSHOVER_USER` (key) and `PUSHOVER_APP` (token)
/// environment variables set in order for the notification to work.
/// 
/// On macOS a desktop notification will also be displayed.
pub fn notify() {
	if let Ok(token) = env::var("PUSHOVER_APP") {
		if let Ok(user_key) = env::var("PUSHOVER_USER") {
			let api = API::new();

			let msg = SendMessage::new(token, user_key, format!("New KEV Release! {}", KEV_CATALOG_URL));

			#[cfg(target_os="macos")]
			{
				let bundle = get_bundle_identifier_or_default("com.apple.Terminal");
				set_application(&bundle).unwrap();
				let _ = send_notification("New KEV Release!", None, format!("Visit {} for more info.", KEV_CATALOG_URL).as_str(), None).unwrap();
			}
			
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

/// At first launch, this will cache the current KEV JSON. Subsequent launches will
/// then compare the current catalog served from CISA's site with the cached one
/// and both update the local cache and fire off a notification. macOS users will also
/// receive a desktop notification.
/// 
/// On macOS and linux, 
/// [`XDG_CACHE_HOME`](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
/// is used as the base cache directory, so the cache file is at `~/.cache/kev-cache/kev.json`.
/// 
/// On Windows the base cache directory is `%LOCALAPPDATA%`,
/// so the cache file is at (`C:\\Users\\%USERNAME%\\AppData\\Local\\kev-cache\\kev.json`).
pub fn run() -> Result<(), Error> {
	let app_dirs: AppDirs = AppDirs::new(Some("kev-cache"), true).expect("Error idenfitying app dir");

	let kev_cache_file_path: PathBuf = app_dirs.cache_dir.join("kev.json");

	// this will create our app cache directory if it does not exist
	fs::create_dir_all(&app_dirs.cache_dir)?;

	if kev_cache_file_path.is_file() {
		// we found a local KEV cached JSON file

		let old_kev: Kev = read_kev_cache_from_file(kev_cache_file_path.as_path())?;
		let new_kev: Kev = read_kev_from_cisa()?;

		if old_kev.date_released != new_kev.date_released {
			// new one is different

			let kev_cache: File = create_kev_cache_file(kev_cache_file_path)?;

			serde_json::to_writer_pretty(&kev_cache, &new_kev)?;

			notify();
		}
	} else {
		// We didn't, so make one!

		let kev: Kev = read_kev_from_cisa()?;
		let kev_cache: File = create_kev_cache_file(kev_cache_file_path)?;

		serde_json::to_writer_pretty(&kev_cache, &kev)?
	};

	Ok(())

}