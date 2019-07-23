use chrono::Utc;
use std::env;
use clap::App;

mod lib;

fn main() {
  let matches = App::new("This is a test")
    .get_matches();

  lib::main(&0i64, Utc::today(), &mut std::io::stdout());

//    Ok(())
}
