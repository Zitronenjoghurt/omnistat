use crate::apis::open_meteo::utils::{parse_iso8601_date, parse_iso8601_datetime};
use crate::error::{IntegrationError, IntegrationResult};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use omnistat_core::types::angle::Angle;
use omnistat_core::types::area_energy_density::AreaEnergyDensity;
use omnistat_core::types::latitude::Latitude;
use omnistat_core::types::length::Length;
use omnistat_core::types::longitude::Longitude;
use omnistat_core::types::percentage::Percentage;
use omnistat_core::types::speed::Speed;
use omnistat_core::types::temperature::Temperature;
use omnistat_core::types::uv_index::UVIndex;
use omnistat_core::types::wmo_code::WMOCode;
use serde::Deserialize;
use std::str::FromStr;
use std::time::Duration;

/// Source: https://open-meteo.com/en/docs
#[derive(Debug)]
pub struct OpenMeteoDaily {
    pub time: NaiveDate,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub elevation: Length,
    /// The most severe weather condition on a given day
    pub wmo_code: WMOCode,
    pub temperature_2m_max: Temperature,
    pub temperature_2m_mean: Temperature,
    pub temperature_2m_min: Temperature,
    pub apparent_temperature_max: Temperature,
    pub apparent_temperature_mean: Temperature,
    pub apparent_temperature_min: Temperature,
    pub precipitation_sum: Length,
    pub rain_sum: Length,
    pub showers_sum: Length,
    pub snowfall_sum: Length,
    pub precipitation_hours: f32,
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

#[derive(Deserialize)]
pub(crate) struct DailyForecastModel {
    pub latitude: f32,
    pub longitude: f32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f32,
    pub daily: DailyForecastDailyModel,
}

impl DailyForecastModel {
    pub fn parse_forecasts(&self) -> IntegrationResult<Vec<OpenMeteoDaily>> {
        let timezone = Tz::from_str(self.timezone.as_str())?;
        let latitude = Latitude::new(self.latitude);
        let longitude = Longitude::new(self.longitude);
        let elevation = Length::from_meters(self.elevation as f64);

        let count = self.daily.time.len();
        let mut forecasts = Vec::with_capacity(count);

        for i in 0..count {
            let time = parse_iso8601_date(&self.daily.time[i])?;
            let wmo_code: WMOCode = self.daily.weather_code[i].into();

            let sunrise_naive = parse_iso8601_datetime(&self.daily.sunrise[i])?;
            let sunrise = timezone
                .from_local_datetime(&sunrise_naive)
                .single()
                .ok_or(IntegrationError::AmbiguousTimezone(self.timezone.clone()))?
                .to_utc();
            let sunset_naive = parse_iso8601_datetime(&self.daily.sunset[i])?;
            let sunset = timezone
                .from_local_datetime(&sunset_naive)
                .single()
                .ok_or(IntegrationError::AmbiguousTimezone(self.timezone.clone()))?
                .to_utc();

            let forecast = OpenMeteoDaily {
                time,
                latitude,
                longitude,
                elevation,
                wmo_code,
                temperature_2m_max: Temperature::from_celsius(self.daily.temperature_2m_max[i]),
                temperature_2m_mean: Temperature::from_celsius(self.daily.temperature_2m_mean[i]),
                temperature_2m_min: Temperature::from_celsius(self.daily.temperature_2m_min[i]),
                apparent_temperature_max: Temperature::from_celsius(
                    self.daily.apparent_temperature_max[i],
                ),
                apparent_temperature_mean: Temperature::from_celsius(
                    self.daily.apparent_temperature_mean[i],
                ),
                apparent_temperature_min: Temperature::from_celsius(
                    self.daily.apparent_temperature_min[i],
                ),
                precipitation_sum: Length::from_millimeters(self.daily.precipitation_sum[i] as f64),
                rain_sum: Length::from_millimeters(self.daily.rain_sum[i] as f64),
                showers_sum: Length::from_millimeters(self.daily.showers_sum[i] as f64),
                snowfall_sum: Length::from_centimeters(self.daily.snowfall_sum[i] as f64),
                precipitation_hours: self.daily.precipitation_hours[i],
                precipitation_probability_max: Percentage::from_0_100(
                    self.daily.precipitation_probability_max[i] as f32,
                ),
                precipitation_probability_mean: Percentage::from_0_100(
                    self.daily.precipitation_probability_mean[i] as f32,
                ),
                precipitation_probability_min: Percentage::from_0_100(
                    self.daily.precipitation_probability_min[i] as f32,
                ),
                sunrise,
                sunset,
                sunshine_duration: Duration::from_secs_f32(self.daily.sunshine_duration[i]),
                daylight_duration: Duration::from_secs_f32(self.daily.daylight_duration[i]),
                wind_speed_10m_max: Speed::from_km_h(self.daily.wind_speed_10m_max[i]),
                wind_gusts_10m_max: Speed::from_km_h(self.daily.wind_gusts_10m_max[i]),
                wind_direction_10m_dominant: Angle::from_degrees(
                    self.daily.wind_direction_10m_dominant[i] as f32,
                ),
                uv_index_max: UVIndex::new(self.daily.uv_index_max[i]),
                uv_index_clear_sky_max: UVIndex::new(self.daily.uv_index_clear_sky_max[i]),
                shortwave_radiation_sum: AreaEnergyDensity::from_mj_m2(
                    self.daily.shortwave_radiation_sum[i],
                ),
            };

            forecasts.push(forecast);
        }

        Ok(forecasts)
    }
}

#[derive(Deserialize)]
pub(crate) struct DailyForecastDailyModel {
    pub time: Vec<String>,
    pub weather_code: Vec<u8>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_mean: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub apparent_temperature_max: Vec<f32>,
    pub apparent_temperature_mean: Vec<f32>,
    pub apparent_temperature_min: Vec<f32>,
    pub uv_index_max: Vec<f32>,
    pub uv_index_clear_sky_max: Vec<f32>,
    pub sunshine_duration: Vec<f32>,
    pub daylight_duration: Vec<f32>,
    pub sunset: Vec<String>,
    pub sunrise: Vec<String>,
    pub rain_sum: Vec<f32>,
    pub showers_sum: Vec<f32>,
    pub snowfall_sum: Vec<f32>,
    pub precipitation_sum: Vec<f32>,
    pub precipitation_hours: Vec<f32>,
    pub precipitation_probability_max: Vec<u8>,
    pub precipitation_probability_mean: Vec<u8>,
    pub precipitation_probability_min: Vec<u8>,
    pub wind_speed_10m_max: Vec<f32>,
    pub wind_gusts_10m_max: Vec<f32>,
    pub wind_direction_10m_dominant: Vec<u16>,
    pub shortwave_radiation_sum: Vec<f32>,
    pub cloud_cover_max: Vec<f32>,
    pub cloud_cover_mean: Vec<f32>,
    pub cloud_cover_min: Vec<f32>,
    pub dew_point_2m_max: Vec<f32>,
    pub dew_point_2m_mean: Vec<f32>,
    pub dew_point_2m_min: Vec<f32>,
    pub relative_humidity_2m_max: Vec<f32>,
    pub relative_humidity_2m_mean: Vec<f32>,
    pub relative_humidity_2m_min: Vec<f32>,
    pub surface_pressure_max: Vec<f32>,
    pub surface_pressure_mean: Vec<f32>,
    pub surface_pressure_min: Vec<f32>,
    pub visibility_max: Vec<f32>,
    pub visibility_mean: Vec<f32>,
    pub visibility_min: Vec<f32>,
}
