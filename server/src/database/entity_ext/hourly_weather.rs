use crate::database::entity::hourly_weather;
use omnistat_core::apis::open_meteo::hourly_forecast::HourlyForecast;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

impl hourly_weather::Entity {
    pub async fn upsert(
        active_model: hourly_weather::ActiveModel,
        connection: &DatabaseConnection,
    ) -> anyhow::Result<()> {
        Self::insert(active_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    hourly_weather::Column::UserId,
                    hourly_weather::Column::TimeUtc,
                ])
                .update_columns([
                    hourly_weather::Column::WmoCode,
                    hourly_weather::Column::TemperatureActual,
                    hourly_weather::Column::TemperatureApparent,
                    hourly_weather::Column::RelativeHumidity,
                    hourly_weather::Column::DewPoint,
                    hourly_weather::Column::SurfacePressure,
                    hourly_weather::Column::CloudCover,
                    hourly_weather::Column::CloudCoverLow,
                    hourly_weather::Column::CloudCoverMid,
                    hourly_weather::Column::CloudCoverHigh,
                    hourly_weather::Column::WindSpeed,
                    hourly_weather::Column::MaxWindSpeed,
                    hourly_weather::Column::WindDirection,
                    hourly_weather::Column::TotalPrecipitation,
                    hourly_weather::Column::PrecipitationProbability,
                    hourly_weather::Column::Rain,
                    hourly_weather::Column::Snowfall,
                    hourly_weather::Column::SnowDepth,
                    hourly_weather::Column::Showers,
                    hourly_weather::Column::Visibility,
                    hourly_weather::Column::ShortwaveRadiation,
                ])
                .to_owned(),
            )
            .exec(connection)
            .await?;
        Ok(())
    }
}

impl hourly_weather::ActiveModel {
    pub fn from_forecast(forecast: &HourlyForecast, user_id: &str) -> Self {
        hourly_weather::ActiveModel {
            user_id: Set(user_id.to_string()),
            time_utc: Set(forecast.time.naive_utc()),
            wmo_code: Set(u8::from(forecast.wmo_code) as i32),
            temperature_actual: Set(forecast.temperature_2m.as_celsius()),
            temperature_apparent: Set(forecast.apparent_temperature.as_celsius()),
            relative_humidity: Set(forecast.relative_humidity_2m.as_0_1()),
            dew_point: Set(forecast.dew_point_2m.as_celsius()),
            surface_pressure: Set(forecast.surface_pressure.as_hpa()),
            cloud_cover: Set(forecast.cloud_cover.as_0_1()),
            cloud_cover_low: Set(forecast.cloud_cover_low.as_0_1()),
            cloud_cover_mid: Set(forecast.cloud_cover_mid.as_0_1()),
            cloud_cover_high: Set(forecast.cloud_cover_high.as_0_1()),
            wind_speed: Set(forecast.wind_speed_10m.as_km_h()),
            max_wind_speed: Set(forecast.max_wind_speed_10m.as_km_h()),
            wind_direction: Set(forecast.wind_direction_10m.as_degrees()),
            total_precipitation: Set(forecast.total_precipitation.as_millimeters()),
            precipitation_probability: Set(forecast.precipitation_probability.as_0_1()),
            rain: Set(forecast.rain.as_millimeters()),
            snowfall: Set(forecast.snowfall.as_millimeters()),
            snow_depth: Set(forecast.snow_depth.as_centimeters()),
            showers: Set(forecast.showers.as_millimeters()),
            visibility: Set(forecast.visibility.as_meters()),
            shortwave_radiation: Set(forecast.shortwave_radiation.as_w_m2()),
        }
    }
}
