#[macro_use]
extern crate clap;

use chrono::Utc;
use std::env;
use clap::{App, Arg};

mod lib;

fn main() {
  let matches = App::new("This is a test")
    .author(crate_authors!())
    .about("Calculates expiration dates of blood components based on a \
      supplied offset and the anticoagulant type")
    .version(crate_version!())
    .arg(Arg::with_name("offset")
      .help("Number of days from today to offset the calculation"))
    .get_matches();

  if let Some(offset) = matches.value_of("offset") {
    lib::main(&offset.parse::<i64>().unwrap(), Utc::today(), &mut std::io::stdout());

    //if let Some(offset_value) = offset.parse::<i64>() {
    //  lib::main(&offset_value, Utc::today(), &mut std::io::stdout());
    //} else {
    //  println!("offset must be a 64 bit intiger");
  } else {
    lib::main(&0i64, Utc::today(), &mut std::io::stdout());
  }

//    Ok(())
}
