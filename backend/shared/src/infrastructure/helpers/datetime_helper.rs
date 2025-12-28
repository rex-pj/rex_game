use std::io::ErrorKind;

use super::datetime_helper_trait::DateTimeHelperTrait;
use chrono::{DateTime, Utc};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct DateTimeHelper {}

impl DateTimeHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl DateTimeHelperTrait for DateTimeHelper {
    fn to_offset_date_time(&self, date_time: DateTime<Utc>) -> Result<OffsetDateTime, ErrorKind> {
        let timestamp = date_time.timestamp(); // seconds since epoch
        let offset_dt = OffsetDateTime::from_unix_timestamp(timestamp);
        match offset_dt {
            Ok(dt) => Ok(dt),
            Err(_) => Err(ErrorKind::InvalidInput),
        }
    }

    fn timestamp_to_offset_date_time(&self, timestamp: u64) -> Result<OffsetDateTime, ErrorKind> {
        if timestamp <= i64::MAX as u64 {
            let offset_dt = OffsetDateTime::from_unix_timestamp(timestamp as i64);
            return match offset_dt {
                Ok(dt) => Ok(dt),
                Err(_) => Err(ErrorKind::InvalidInput),
            };
        }

        Err(ErrorKind::InvalidInput)
    }

    fn timestamp_to_utc_date_time(&self, timestamp: u64) -> Result<DateTime<Utc>, ErrorKind> {
        if timestamp <= i64::MAX as u64 {
            let offset_dt = DateTime::from_timestamp(timestamp as i64, 0);
            return match offset_dt {
                Some(dt) => Ok(dt),
                None => Err(ErrorKind::InvalidInput),
            };
        }

        Err(ErrorKind::InvalidInput)
    }
}
