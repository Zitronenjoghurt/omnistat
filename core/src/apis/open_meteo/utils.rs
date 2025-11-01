use crate::error::OmnistatResult;
use chrono::NaiveDateTime;

pub fn parse_iso8601(value: impl AsRef<str>) -> OmnistatResult<NaiveDateTime> {
    Ok(NaiveDateTime::parse_from_str(
        value.as_ref(),
        "%Y-%m-%dT%H:%M",
    )?)
}
