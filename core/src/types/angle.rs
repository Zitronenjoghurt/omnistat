use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Angle in radians
pub struct Angle(f32);

impl Angle {
    pub fn from_radians(value: f32) -> Self {
        Self(value)
    }

    pub fn from_degrees(value: f32) -> Self {
        Self(value * std::f32::consts::PI / 180.0)
    }

    pub fn as_radians(&self) -> f32 {
        self.0
    }

    pub fn as_degrees(&self) -> f32 {
        self.0 * 180.0 / std::f32::consts::PI
    }

    pub fn format_radians(&self) -> String {
        format!("{:.2} rad", self.as_radians())
    }

    pub fn format_degrees(&self) -> String {
        format!("{:.2}°", self.as_degrees())
    }
}
