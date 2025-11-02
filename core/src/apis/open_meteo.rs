use crate::apis::open_meteo::daily_forecast::DailyForecastModel;
use crate::apis::open_meteo::hourly_forecast::HourlyForecastModel;
use crate::client::ApiClient;
use crate::error::OmnistatResult;
use crate::types::latitude::Latitude;
use crate::types::longitude::Longitude;
use std::time::Duration;

pub mod daily_forecast;
pub mod hourly_forecast;
mod utils;

pub struct OpenMeteoApi {
    client: ApiClient,
}

impl Default for OpenMeteoApi {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenMeteoApi {
    pub fn new() -> Self {
        let client = ApiClient::new(100, 1, Duration::from_secs(30));
        Self { client }
    }

    pub async fn hourly_forecasts(
        &self,
        latitude: Latitude,
        longitude: Longitude,
    ) -> OmnistatResult<Vec<hourly_forecast::OpenMeteoHourly>> {
        let request = self.client.request("https://api.open-meteo.com/v1/forecast?hourly=temperature_2m,relative_humidity_2m,dew_point_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,snow_depth,weather_code,surface_pressure,cloud_cover,cloud_cover_low,cloud_cover_mid,cloud_cover_high,visibility,wind_speed_10m,wind_speed_80m,wind_speed_120m,wind_speed_180m,wind_direction_10m,wind_direction_80m,wind_direction_120m,wind_direction_180m,wind_gusts_10m,shortwave_radiation&forecast_days=7")?
            .query("latitude", latitude.value().to_string())
            .query("longitude", longitude.value().to_string())
            .query("timezone", "auto");
        let model: HourlyForecastModel = request.get_json().await?;
        model.parse_forecasts()
    }

    pub async fn daily_forecasts(
        &self,
        latitude: Latitude,
        longitude: Longitude,
    ) -> OmnistatResult<Vec<daily_forecast::OpenMeteoDaily>> {
        let request = self.client.request("https://api.open-meteo.com/v1/forecast?daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,uv_index_clear_sky_max,uv_index_max,sunshine_duration,daylight_duration,sunset,sunrise,rain_sum,showers_sum,snowfall_sum,precipitation_sum,precipitation_hours,precipitation_probability_max,wind_speed_10m_max,wind_gusts_10m_max,wind_direction_10m_dominant,shortwave_radiation_sum,temperature_2m_mean,apparent_temperature_mean,cloud_cover_mean,cloud_cover_max,cloud_cover_min,dew_point_2m_mean,dew_point_2m_max,dew_point_2m_min,precipitation_probability_mean,precipitation_probability_min,relative_humidity_2m_mean,relative_humidity_2m_max,relative_humidity_2m_min,surface_pressure_mean,surface_pressure_max,surface_pressure_min,visibility_mean,visibility_max,visibility_min&forecast_days=7")?
            .query("latitude", latitude.value().to_string())
            .query("longitude", longitude.value().to_string())
            .query("timezone", "auto");
        let model: DailyForecastModel = request.get_json().await?;
        model.parse_forecasts()
    }
}
