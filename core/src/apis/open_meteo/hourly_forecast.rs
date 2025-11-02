use crate::apis::open_meteo::utils::parse_iso8601_datetime;
use crate::error::{OmnistatError, OmnistatResult};
use crate::types::angle::Angle;
use crate::types::area_power_density::AreaPowerDensity;
use crate::types::latitude::Latitude;
use crate::types::length::Length;
use crate::types::longitude::Longitude;
use crate::types::percentage::Percentage;
use crate::types::pressure::Pressure;
use crate::types::speed::Speed;
use crate::types::temperature::Temperature;
use crate::types::wmo_code::WMOCode;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::Deserialize;
use std::str::FromStr;

/// Source: https://open-meteo.com/en/docs
#[derive(Debug)]
pub struct OpenMeteoHourly {
    pub time: DateTime<Utc>,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub wmo_code: WMOCode,
    pub elevation: Length,
    /// Apparent temperature is the perceived feels-like temperature combining wind chill factor, relative humidity and solar radiation
    pub apparent_temperature: Temperature,
    /// Air temperature at 2 meters above ground
    pub temperature_2m: Temperature,
    /// Relative humidity at 2 meters above ground
    pub relative_humidity_2m: Percentage,
    /// Dew point temperature at 2 meters above ground
    pub dew_point_2m: Temperature,
    /// Atmospheric air pressure at surface level
    pub surface_pressure: Pressure,
    /// Total cloud cover
    pub cloud_cover: Percentage,
    /// Low-level clouds and fog up to 3 km altitude
    pub cloud_cover_low: Percentage,
    /// Mid-level clouds from 3 to 8 km altitude
    pub cloud_cover_mid: Percentage,
    /// High-level clouds from 8 km altitude
    pub cloud_cover_high: Percentage,
    pub wind_speed_10m: Speed,
    pub wind_speed_80m: Speed,
    pub wind_speed_120m: Speed,
    pub wind_speed_180m: Speed,
    pub wind_direction_10m: Angle,
    pub wind_direction_80m: Angle,
    pub wind_direction_120m: Angle,
    pub wind_direction_180m: Angle,
    pub max_wind_speed_10m: Speed,
    /// Total precipitation (rain, showers, snow) sum of the preceding hour
    pub total_precipitation: Length,
    /// Probability of precipitation with more than 0.1 mm of the preceding hour. Probability is based on ensemble weather models with 0.25° (~27 km) resolution. 30 different simulations are computed to better represent future weather conditions.
    pub precipitation_probability: Percentage,
    /// Rain from large scale weather systems of the preceding hour
    pub rain: Length,
    /// Snowfall amount of the preceding hour in centimeters. For the water equivalent in millimeter, divide by 7. E.g. 7 cm snow = 10 mm precipitation water equivalent
    pub snowfall: Length,
    pub snow_depth: Length,
    /// Showers from convective precipitation in millimeters from the preceding hour
    pub showers: Length,
    /// Viewing distance in meters. Influenced by low clouds, humidity and aerosols.
    pub visibility: Length,
    /// Shortwave solar radiation as average of the preceding hour. This is equal to the total global horizontal irradiation
    pub shortwave_radiation: AreaPowerDensity,
}

#[derive(Deserialize)]
pub(crate) struct HourlyForecastModel {
    pub latitude: f32,
    pub longitude: f32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f32,
    pub hourly: HourlyForecastHoursModel,
}

impl HourlyForecastModel {
    pub fn parse_forecasts(&self) -> OmnistatResult<Vec<OpenMeteoHourly>> {
        let timezone = Tz::from_str(self.timezone.as_str())?;
        let latitude = Latitude::new(self.latitude);
        let longitude = Longitude::new(self.longitude);
        let elevation = Length::from_meters(self.elevation as f64);

        let count = self.hourly.time.len();
        let mut forecasts = Vec::with_capacity(count);

        for i in 0..count {
            let naive_dt = parse_iso8601_datetime(&self.hourly.time[i])?;
            let time = timezone
                .from_local_datetime(&naive_dt)
                .single()
                .ok_or(OmnistatError::AmbiguousTimezone(self.timezone.clone()))?
                .to_utc();
            let wmo_code: WMOCode = self.hourly.weather_code[i].into();

            let forecast = OpenMeteoHourly {
                time,
                latitude,
                longitude,
                wmo_code,
                elevation,
                apparent_temperature: Temperature::from_celsius(
                    self.hourly.apparent_temperature[i],
                ),
                temperature_2m: Temperature::from_celsius(self.hourly.temperature_2m[i]),
                relative_humidity_2m: Percentage::from_0_100(
                    self.hourly.relative_humidity_2m[i] as f32,
                ),
                dew_point_2m: Temperature::from_celsius(self.hourly.dew_point_2m[i]),
                surface_pressure: Pressure::from_hpa(self.hourly.surface_pressure[i]),
                cloud_cover: Percentage::from_0_100(self.hourly.cloud_cover[i] as f32),
                cloud_cover_low: Percentage::from_0_100(self.hourly.cloud_cover_low[i] as f32),
                cloud_cover_mid: Percentage::from_0_100(self.hourly.cloud_cover_mid[i] as f32),
                cloud_cover_high: Percentage::from_0_100(self.hourly.cloud_cover_high[i] as f32),
                wind_speed_10m: Speed::from_km_h(self.hourly.wind_speed_10m[i]),
                wind_speed_80m: Speed::from_km_h(self.hourly.wind_speed_80m[i]),
                wind_speed_120m: Speed::from_km_h(self.hourly.wind_speed_120m[i]),
                wind_speed_180m: Speed::from_km_h(self.hourly.wind_speed_180m[i]),
                wind_direction_10m: Angle::from_degrees(self.hourly.wind_direction_10m[i] as f32),
                wind_direction_80m: Angle::from_degrees(self.hourly.wind_direction_80m[i] as f32),
                wind_direction_120m: Angle::from_degrees(self.hourly.wind_direction_120m[i] as f32),
                wind_direction_180m: Angle::from_degrees(self.hourly.wind_direction_180m[i] as f32),
                max_wind_speed_10m: Speed::from_km_h(self.hourly.wind_gusts_10m[i]),
                total_precipitation: Length::from_millimeters(self.hourly.precipitation[i] as f64),
                precipitation_probability: Percentage::from_0_100(
                    self.hourly.precipitation_probability[i] as f32,
                ),
                rain: Length::from_millimeters(self.hourly.rain[i] as f64),
                snowfall: Length::from_centimeters(self.hourly.snowfall[i] as f64),
                snow_depth: Length::from_meters(self.hourly.snow_depth[i] as f64),
                showers: Length::from_millimeters(self.hourly.showers[i] as f64),
                visibility: Length::from_meters(self.hourly.visibility[i] as f64),
                shortwave_radiation: AreaPowerDensity::from_w_m2(
                    self.hourly.shortwave_radiation[i],
                ),
            };

            forecasts.push(forecast);
        }

        Ok(forecasts)
    }
}

#[derive(Deserialize)]
pub(crate) struct HourlyForecastHoursModel {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub relative_humidity_2m: Vec<u8>,
    pub dew_point_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
    pub precipitation_probability: Vec<u8>,
    pub precipitation: Vec<f32>,
    pub rain: Vec<f32>,
    pub showers: Vec<f32>,
    pub snowfall: Vec<f32>,
    pub snow_depth: Vec<f32>,
    pub weather_code: Vec<u8>,
    pub surface_pressure: Vec<f32>,
    pub cloud_cover: Vec<u8>,
    pub cloud_cover_low: Vec<u8>,
    pub cloud_cover_mid: Vec<u8>,
    pub cloud_cover_high: Vec<u8>,
    pub visibility: Vec<f32>,
    pub wind_speed_10m: Vec<f32>,
    pub wind_speed_80m: Vec<f32>,
    pub wind_speed_120m: Vec<f32>,
    pub wind_speed_180m: Vec<f32>,
    pub wind_direction_10m: Vec<u16>,
    pub wind_direction_80m: Vec<u16>,
    pub wind_direction_120m: Vec<u16>,
    pub wind_direction_180m: Vec<u16>,
    pub wind_gusts_10m: Vec<f32>,
    pub shortwave_radiation: Vec<f32>,
}
