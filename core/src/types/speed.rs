use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Speed in m/s
pub struct Speed(f32);

impl Speed {
    pub fn from_m_s(value: f32) -> Self {
        Self(value)
    }

    pub fn from_km_h(value: f32) -> Self {
        Self(value / 3.6)
    }

    pub fn as_m_s(&self) -> f32 {
        self.0
    }

    pub fn as_km_h(&self) -> f32 {
        self.0 * 3.6
    }

    pub fn format_m_s(&self) -> String {
        format!("{:.2} m/s", self.as_m_s())
    }

    pub fn format_km_h(&self) -> String {
        format!("{:.2} km/h", self.as_km_h())
    }
}
