extern crate chrono;
use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigaseconds = Duration::seconds(1_000_000_000);
    return start + gigaseconds;
}
