#![cfg(feature = "timext")]
use time::{OffsetDateTime, PrimitiveDateTime};

pub trait OffsetDateTimeExt {
    /// Strip the offset and return the underlying wall-clock date+time.
    ///
    /// This is effectively equivalent to a "naive" datetime in UTC.
    fn to_primitive(self) -> PrimitiveDateTime;
}

impl OffsetDateTimeExt for OffsetDateTime {
    #[inline]
    fn to_primitive(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self.date(), self.time())
    }
}

#[cfg(test)]
mod tests {
    use super::OffsetDateTimeExt;
    use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

    #[test]
    fn test_offset_to_primitive() {
        let dt = OffsetDateTime::from_unix_timestamp(1766496840).unwrap();
        let date = Date::from_calendar_date(2025, Month::December, 23).unwrap();
        let time = Time::from_hms(13, 34, 0).unwrap();
        assert_eq!(dt.to_primitive(), PrimitiveDateTime::new(date, time));
    }
}
