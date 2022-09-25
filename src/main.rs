use anyhow::{Result, Error};

mod lib;

fn main() -> Result<(), Error> {

	let _ = lib::run();

	Ok(())

}
