use crate::apis::open_meteo::hourly_forecast::HourlyForecastModel;
use crate::client::ApiClient;
use crate::error::OmnistatResult;
use crate::types::latitude::Latitude;
use crate::types::longitude::Longitude;
use std::time::Duration;

mod daily_forecast;
mod hourly_forecast;
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
        timezone: impl Into<String>,
    ) -> OmnistatResult<Vec<hourly_forecast::HourlyForecast>> {
        let request = self.client.request("https://api.open-meteo.com/v1/forecast?hourly=temperature_2m,relative_humidity_2m,dew_point_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,snow_depth,weather_code,surface_pressure,cloud_cover,cloud_cover_low,cloud_cover_mid,cloud_cover_high,visibility,wind_speed_10m,wind_speed_80m,wind_speed_120m,wind_speed_180m,wind_direction_10m,wind_direction_80m,wind_direction_120m,wind_direction_180m,wind_gusts_10m,temperature_80m,temperature_120m,temperature_180m,shortwave_radiation&forecast_days=1")?
            .query("latitude", latitude.value().to_string())
            .query("longitude", longitude.value().to_string())
            .query("timezone", timezone.into());
        let model: HourlyForecastModel = request.get_json().await?;
        model.parse_forecasts()
    }
}
