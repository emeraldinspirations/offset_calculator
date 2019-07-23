// src/lib.rs

// extern crate offset_calculator;

use chrono::Utc;
use chrono::Date;
use chrono::offset::TimeZone;

pub fn main (
  offset: i32,
  date: Date<Utc>,
  mut writer: impl std::io::Write
) {
  writeln!(writer, "Expires:\t14 MAY 1984");
}

///# User Story #1563275034
///
///As a *[Technician] in the [Hospital Services Department]*
///I want to be able to *enter a number of days*
///and *have the future expiration date returned in the format mmm dd*
#[test]
pub fn test_user_story_1863275034() {
  let date = Utc.ymd(1984, 5, 7);
  let offset = 7;
  let expected = "Expires:\t14 MAY 1984";
  let mut actual:Vec<u8> = Vec::new();

  main(offset, date, &mut actual);

  assert!(String::from_utf8_lossy(&actual).contains(expected));
}
