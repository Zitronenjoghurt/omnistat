use crate::types::angle::Angle;
use crate::types::area_energy_density::AreaEnergyDensity;
use crate::types::latitude::Latitude;
use crate::types::length::Length;
use crate::types::longitude::Longitude;
use crate::types::percentage::Percentage;
use crate::types::speed::Speed;
use crate::types::temperature::Temperature;
use crate::types::uv_index::UVIndex;
use crate::types::wmo_code::WMOCode;
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Source: https://open-meteo.com/en/docs
pub struct DailyForecast {
    pub time: DateTime<Utc>,
    pub latitude: Latitude,
    pub longitude: Longitude,
    /// The most severe weather condition on a given day
    pub wmo_code: WMOCode,
    pub temperature_2m_max: Temperature,
    pub temperature_2m_mean: Temperature,
    pub temperature_2m_min: Temperature,
    pub apparent_temperature_2m_max: Temperature,
    pub apparent_temperature_2m_mean: Temperature,
    pub apparent_temperature_2m_min: Temperature,
    pub precipitation_sum: Length,
    pub rain_sum: Length,
    pub showers_sum: Length,
    pub snowfall_sum: Length,
    pub precipitation_hours: u8,
    pub precipitation_probability_max: Percentage,
    pub precipitation_probability_mean: Percentage,
    pub precipitation_probability_min: Percentage,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    /// The duration sunshine per day is determined by calculating direct normalized irradiance exceeding 120 W/m², following the WMO definition. Sunshine duration will consistently be less than daylight duration due to dawn and dusk.
    pub sunshine_duration: Duration,
    pub daylight_duration: Duration,
    pub wind_speed_10m_max: Speed,
    pub wind_gusts_10m_max: Speed,
    pub wind_direction_10m_dominant: Angle,
    pub uv_index_max: UVIndex,
    pub uv_index_clear_sky_max: UVIndex,
    /// The sum of solar radiation on a given day
    pub shortwave_radiation_sum: AreaEnergyDensity,
}
