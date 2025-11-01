use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Pressure in hPa
pub struct Pressure(f32);

impl Pressure {
    pub fn from_hpa(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kpa(value: f32) -> Self {
        Self(value * 10.0)
    }

    pub fn from_bar(value: f32) -> Self {
        Self(value * 1000.0)
    }

    pub fn as_hpa(&self) -> f32 {
        self.0
    }

    pub fn as_kpa(&self) -> f32 {
        self.0 / 10.0
    }

    pub fn as_bar(&self) -> f32 {
        self.0 / 1000.0
    }

    pub fn format_hpa(&self) -> String {
        format!("{:.2} hPa", self.as_hpa())
    }

    pub fn format_kpa(&self) -> String {
        format!("{:.2} kPa", self.as_kpa())
    }

    pub fn format_bar(&self) -> String {
        format!("{:.2} bar", self.as_bar())
    }
}
