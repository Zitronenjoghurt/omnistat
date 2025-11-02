use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Percentage in 0.0-1.0 range
pub struct Percentage(f32);

impl Percentage {
    pub fn from_0_1(value: f32) -> Self {
        Self(value)
    }

    pub fn from_0_100(value: f32) -> Self {
        Self(value / 100.0)
    }

    pub fn as_0_1(self) -> f32 {
        self.0
    }

    pub fn as_0_100(self) -> f32 {
        self.0 * 100.0
    }
}
