// src/lib.rs

// extern crate offset_calculator;

use chrono::Utc;
use chrono::Date;
use chrono::Duration;
use chrono::offset::TimeZone;

pub fn main (
  offset: &i64,
  date: Date<Utc>,
  mut writer: impl std::io::Write
) {

  let date_text =
    (date + Duration::days(*offset))
    .format("%d %b %Y")
    .to_string()
    .to_uppercase();

  writeln!(writer, "Expires:\t{}", date_text);
}

///# User Story #1563275034
///
///As a *[Technician] in the [Hospital Services Department]*
///I want to be able to *enter a number of days*
///and *have the future expiration date returned in the format mmm dd*
#[test]
pub fn test_user_story_1863275034() {
  let date = Utc.ymd(1984, 5, 7);
  let scenario_collection = [
    (7i64, "Expires:\t14 MAY 1984"),
    (8i64, "Expires:\t15 MAY 1984"),
  ];

  for scenario in scenario_collection.iter() {
    let (offset, expected) = scenario;
    let mut actual:Vec<u8> = Vec::new();

    main(offset, date, &mut actual);

    let result = String::from_utf8_lossy(&actual);
    assert!(String::from_utf8_lossy(&actual).contains(expected));
  }
}
