use std::string::ToString;

pub struct ScannerSubscription {
    pub number_of_rows: i32,
    pub instrument: String,
    pub location_code: String,
    pub scan_code: String,
    pub above_price: f64,
    pub below_price: f64,
    pub above_volume: i32,
    pub market_cap_above: f64,
    pub market_cap_below: f64,
    pub moody_rating_above: String,
    pub moody_rating_below: String,
    pub sp_rating_above: String,
    pub sp_rating_below: String,
    pub maturity_date_above: String,
    pub maturity_date_below: String,
    pub coupon_rate_above: f64,
    pub coupon_rate_below: f64,
    pub exclude_convertible: bool,
    pub average_option_volume_above: i32,
    pub scanner_setting_pairs: String,
    pub stock_type_filter: String,
}

impl ScannerSubscription {
    pub fn new() -> ScannerSubscription {
        ScannerSubscription {
            number_of_rows: -1,
            instrument: "".to_string(),
            location_code: "".to_string(),
            scan_code: "".to_string(),
            above_price: f64::MAX,
            below_price: f64::MAX,
            above_volume: i32::MAX,
            market_cap_above: f64::MAX,
            market_cap_below: f64::MAX,
            moody_rating_above: "".to_string(),
            moody_rating_below: "".to_string(),
            sp_rating_above: "".to_string(),
            sp_rating_below: "".to_string(),
            maturity_date_above: "".to_string(),
            maturity_date_below: "".to_string(),
            coupon_rate_above: f64::MAX,
            coupon_rate_below: f64::MAX,
            exclude_convertible: false,
            average_option_volume_above: i32::MAX,
            scanner_setting_pairs: "".to_string(),
            stock_type_filter: "".to_string()
        }
    }
}
