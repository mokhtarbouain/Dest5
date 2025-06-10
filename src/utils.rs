

use chrono::{DateTime, Utc, NaiveDateTime, ParseError};
use chrono::format::ParseResult;

pub fn is_image_taken_during_day(date_time: &str) -> Result<bool, ParseError> {
    let dt: ParseResult<DateTime<Utc>> = DateTime::parse_from_rfc3339(date_time);
    match dt {
        Ok(dt) => {
            let hour = dt.hour();
            Ok(hour >= 6 && hour < 18)
        },
        Err(e) => Err(e),
    }
}