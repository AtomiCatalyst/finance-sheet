use sheets4::{api::{UpdateValuesResponse, ValueRange}, hyper, hyper_rustls, Error, Sheets};

use crate::config::Config;

pub async fn read(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    return hub
        .spreadsheets()
        .values_get(&config.sheet_id, &config.deposit_range_input)
        .doit()
        .await;
}

pub async fn write(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> Result<(hyper::Response<hyper::Body>, UpdateValuesResponse), Error> {
    let new_vals: ValueRange = ValueRange::default();
    return hub
        .spreadsheets()
        .values_update(new_vals, &config.sheet_id, &config.deposit_range_input)
        .value_input_option("voluptua.")
        .doit()
        .await;
}
