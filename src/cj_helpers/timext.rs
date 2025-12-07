#![cfg(feature = "timext")]
use time::{OffsetDateTime, PrimitiveDateTime};

/// Extension helpers for `time::OffsetDateTime` behind the `timext` feature.
///
/// This trait provides small, focused utilities for working with
/// `OffsetDateTime` values. The primary helper, [`to_primitive`], discards the
/// offset information and returns just the wall‑clock date and time as a
/// `PrimitiveDateTime` (also called a "naive" datetime in many libraries).
///
/// Notes and caveats:
/// - No timezone conversion is performed. The date and time are taken directly
///   from the `OffsetDateTime`'s components (which, for the `time` crate,
///   represent the UTC calendar/time components). The offset is simply not
///   carried forward.
/// - This operation is lossy: the original offset is discarded. Only use this
///   when you intentionally want to ignore offset/zone information, for example
///   when storing a UTC‑naive timestamp in a database field that does not carry
///   timezone, or when comparing only wall‑clock components.
/// - If you need a particular timezone representation, convert first (e.g., to
///   UTC or local offset) and then call [`to_primitive`].
pub trait OffsetDateTimeExt {
    /// Return the wall‑clock date and time with no offset attached.
    ///
    /// This strips the offset from an `OffsetDateTime` and produces a
    /// `PrimitiveDateTime` using the same calendar date and clock time. No
    /// conversion is performed; offset/zone information is dropped.
    ///
    /// Example
    /// ```
    /// use time::{OffsetDateTime, Date, Month, Time, PrimitiveDateTime};
    /// use cj_common::cj_helpers::timext::OffsetDateTimeExt; // path may vary
    ///
    /// // 2025-12-23T13:34:00Z
    /// let dt = OffsetDateTime::from_unix_timestamp(1766496840).unwrap();
    /// let naive = dt.to_primitive();
    /// assert_eq!(
    ///     naive,
    ///     PrimitiveDateTime::new(
    ///         Date::from_calendar_date(2025, Month::December, 23).unwrap(),
    ///         Time::from_hms(13, 34, 0).unwrap()
    ///     )
    /// );
    /// ```
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
