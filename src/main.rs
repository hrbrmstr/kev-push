mod lib;

use std::fs::{self, File};
use std::path::PathBuf;

use anyhow::{Result, Error};

use platform_dirs::AppDirs;

fn main() -> Result<(), Error> {

	let app_dirs: AppDirs = AppDirs::new(Some("kev-cache"), true)
	  .expect("Error idenfitying app dir");

  let kev_cache_file_path: PathBuf = app_dirs.cache_dir.join("kev.json");

	// this will create our app cache directory if it does not exist
	fs::create_dir_all(&app_dirs.cache_dir)?;

	if kev_cache_file_path.is_file() { // we found a local KEV cached JSON file
		
    let old_kev: lib::Kev = lib::read_kev_cache_from_file(kev_cache_file_path.as_path())?;
		let new_kev: lib::Kev = lib::read_kev_from_cisa()?;

		if old_kev.date_released != new_kev.date_released {

			let kev_cache: File = lib::create_kev_cache_file(kev_cache_file_path)?;

			serde_json::to_writer_pretty(&kev_cache, &new_kev)?;

			lib::notify();
			
		}

  } else { // We didn't, so make one!

		let kev: lib::Kev = lib::read_kev_from_cisa()?;
		let kev_cache: File = lib::create_kev_cache_file(kev_cache_file_path)?;

		serde_json::to_writer_pretty(&kev_cache, &kev)?

  };

	Ok(())

}
