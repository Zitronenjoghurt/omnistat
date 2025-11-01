use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Length in meters
pub struct Length(f64);

impl Length {
    pub fn from_millimeters(value: f64) -> Self {
        Self(value / 1000.0)
    }

    pub fn from_centimeters(value: f64) -> Self {
        Self(value / 100.0)
    }

    pub fn from_meters(value: f64) -> Self {
        Self(value)
    }

    pub fn as_millimeters(&self) -> f64 {
        self.0 * 1000.0
    }

    pub fn as_centimeters(&self) -> f64 {
        self.0 * 100.0
    }

    pub fn as_meters(&self) -> f64 {
        self.0
    }

    pub fn format_millimeters(&self) -> String {
        format!("{:.2} mm", self.as_millimeters())
    }

    pub fn format_centimeters(&self) -> String {
        format!("{:.2} cm", self.as_centimeters())
    }

    pub fn format_meters(&self) -> String {
        format!("{:.2} m", self.as_meters())
    }
}
