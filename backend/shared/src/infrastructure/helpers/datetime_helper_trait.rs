use std::io::ErrorKind;

use chrono::{DateTime, Utc};
use time::OffsetDateTime;

pub trait DateTimeHelperTrait {
    fn to_offset_date_time(&self, date_time: DateTime<Utc>) -> Result<OffsetDateTime, ErrorKind>;
    fn timestamp_to_offset_date_time(&self, timestamp: u64) -> Result<OffsetDateTime, ErrorKind>;
    fn timestamp_to_utc_date_time(&self, timestamp: u64) -> Result<DateTime<Utc>, ErrorKind>;
}
