pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
    pub deposit_range_input: String,
    pub deposit_range_output: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            priv_key: String::from("priv_key.json"),
            sheet_id: String::from("1nYamm_hb-G8bIfSTY88hz_PnyvmBOMIhcg4kKVDoai8"),
            deposit_range_input: String::from("Sheet1!A2:B"),
            deposit_range_output: String::from("Sheet1!G1"),
        }
    }
}
