use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HourlyWeather::Table)
                    .if_not_exists()
                    .col(string(HourlyWeather::UserId))
                    .col(timestamp(HourlyWeather::TimeUtc))
                    .col(integer(HourlyWeather::WmoCode))
                    .col(float(HourlyWeather::TemperatureActual))
                    .col(float(HourlyWeather::TemperatureApparent))
                    .col(float(HourlyWeather::RelativeHumidity))
                    .col(float(HourlyWeather::DewPoint))
                    .col(float(HourlyWeather::SurfacePressure))
                    .col(float(HourlyWeather::CloudCover))
                    .col(float(HourlyWeather::CloudCoverLow))
                    .col(float(HourlyWeather::CloudCoverMid))
                    .col(float(HourlyWeather::CloudCoverHigh))
                    .col(float(HourlyWeather::WindSpeed))
                    .col(float(HourlyWeather::MaxWindSpeed))
                    .col(float(HourlyWeather::WindDirection))
                    .col(double(HourlyWeather::TotalPrecipitation))
                    .col(float(HourlyWeather::PrecipitationProbability))
                    .col(double(HourlyWeather::Rain))
                    .col(double(HourlyWeather::Snowfall))
                    .col(double(HourlyWeather::SnowDepth))
                    .col(double(HourlyWeather::Showers))
                    .col(double(HourlyWeather::Visibility))
                    .col(float(HourlyWeather::ShortwaveRadiation))
                    .primary_key(
                        Index::create()
                            .col(HourlyWeather::UserId)
                            .col(HourlyWeather::TimeUtc),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HourlyWeather::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HourlyWeather {
    Table,
    UserId,
    TimeUtc,
    WmoCode,
    TemperatureActual,
    TemperatureApparent,
    RelativeHumidity,
    DewPoint,
    SurfacePressure,
    CloudCover,
    CloudCoverLow,
    CloudCoverMid,
    CloudCoverHigh,
    WindSpeed,
    MaxWindSpeed,
    WindDirection,
    TotalPrecipitation,
    PrecipitationProbability,
    Rain,
    Snowfall,
    SnowDepth,
    Showers,
    Visibility,
    ShortwaveRadiation,
}
