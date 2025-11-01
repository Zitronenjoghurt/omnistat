#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
/// Temperature in Kelvin
pub struct Temperature(f32);

impl Temperature {
    pub fn from_kelvin(value: f32) -> Self {
        Self(value)
    }

    pub fn from_celsius(value: f32) -> Self {
        Self(value + 273.15)
    }

    pub fn from_fahrenheit(value: f32) -> Self {
        Self((value + 459.67) * 5.0 / 9.0)
    }

    pub fn as_kelvin(&self) -> f32 {
        self.0
    }

    pub fn as_celsius(&self) -> f32 {
        self.0 - 273.15
    }

    pub fn as_fahrenheit(&self) -> f32 {
        (self.0 * 9.0 / 5.0) - 459.67
    }

    pub fn format_kelvin(&self) -> String {
        format!("{:.2} K", self.0)
    }

    pub fn format_celsius(&self) -> String {
        format!("{:.2} °C", self.as_celsius())
    }

    pub fn format_fahrenheit(&self) -> String {
        format!("{:.2} °F", self.as_fahrenheit())
    }
}
