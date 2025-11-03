use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// Digital information in bytes
pub struct DigitalInformation(usize);

impl DigitalInformation {
    pub fn from_bytes(value: usize) -> Self {
        Self(value)
    }

    pub fn from_kb(value: usize) -> Self {
        Self(value.saturating_mul(1_000))
    }

    pub fn from_mb(value: usize) -> Self {
        Self(value.saturating_mul(1_000_000))
    }

    pub fn from_gb(value: usize) -> Self {
        Self(value.saturating_mul(1_000_000_000))
    }

    pub fn from_tb(value: usize) -> Self {
        Self(value.saturating_mul(1_000_000_000_000))
    }

    pub fn as_bytes(&self) -> f64 {
        self.0 as f64
    }

    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / 1_000.0
    }

    pub fn as_mb(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    pub fn as_gb(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }

    pub fn as_tb(&self) -> f64 {
        self.0 as f64 / 1_000_000_000_000.0
    }

    pub fn format_pretty(&self) -> String {
        if self.as_bytes() < 1_000.0 {
            format!("{} B", self.as_bytes())
        } else if self.as_kb() < 1_000.0 {
            format!("{:.2} kB", self.as_kb())
        } else if self.as_mb() < 1_000.0 {
            format!("{:.2} MB", self.as_mb())
        } else if self.as_gb() < 1_000.0 {
            format!("{:.2} GB", self.as_gb())
        } else {
            format!("{:.2} TB", self.as_tb())
        }
    }
}
