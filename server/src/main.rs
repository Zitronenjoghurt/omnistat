use omnistat_core::apis::open_meteo::OpenMeteoApi;
use omnistat_core::types::latitude::Latitude;
use omnistat_core::types::longitude::Longitude;

#[tokio::main]
async fn main() {
    let open_meteo = OpenMeteoApi::new();
    let results = open_meteo
        .daily_forecasts(Latitude::new(50.0), Longitude::new(10.0))
        .await
        .unwrap();
    println!("{:#?}", results);
}
