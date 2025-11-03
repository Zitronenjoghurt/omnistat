use crate::error::IntegrationResult;
use chrono::{NaiveDate, NaiveDateTime};

pub fn parse_iso8601_datetime(value: impl AsRef<str>) -> IntegrationResult<NaiveDateTime> {
    Ok(NaiveDateTime::parse_from_str(
        value.as_ref(),
        "%Y-%m-%dT%H:%M",
    )?)
}

pub fn parse_iso8601_date(value: impl AsRef<str>) -> IntegrationResult<NaiveDate> {
    Ok(NaiveDate::parse_from_str(value.as_ref(), "%Y-%m-%d")?)
}
