use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Power density in W/m^2
pub struct AreaPowerDensity(f32);

impl AreaPowerDensity {
    pub fn from_w_m2(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kw_m2(value: f32) -> Self {
        Self(value * 1_000.0)
    }

    pub fn as_w_m2(&self) -> f32 {
        self.0
    }

    pub fn as_kw_m2(&self) -> f32 {
        self.0 / 1_000.0
    }

    pub fn format_w_m2(&self) -> String {
        format!("{:.2} W/m²", self.as_w_m2())
    }

    pub fn format_kw_m2(&self) -> String {
        format!("{:.2} kW/m²", self.as_kw_m2())
    }
}
