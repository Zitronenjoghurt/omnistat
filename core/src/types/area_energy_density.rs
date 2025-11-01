use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Energy density in J/m^2
pub struct AreaEnergyDensity(f32);

impl AreaEnergyDensity {
    pub fn from_j_m2(value: f32) -> Self {
        Self(value)
    }

    pub fn from_mj_m2(value: f32) -> Self {
        Self(value / 1_000_000.0)
    }

    pub fn as_j_m2(&self) -> f32 {
        self.0
    }

    pub fn as_mj_m2(&self) -> f32 {
        self.0 * 1_000_000.0
    }

    pub fn format_j_m2(&self) -> String {
        format!("{:.2} J/m^2", self.as_j_m2())
    }

    pub fn format_mj_m2(&self) -> String {
        format!("{:.2} MJ/m^2", self.as_mj_m2())
    }
}
