#[derive(Debug)]
pub struct OrderStatusMessage {
    pub order_id: i32,
    pub status: String,
    pub filled: f64,
    pub remaining: f64,
    pub avg_fill_price: f64,
    pub perm_id: i32,
    pub parent_id: i32,
    pub last_fill_price: f64,
    pub client_id: i32,
    pub why_held: String,
    pub mkt_cap_price: f64,
}

impl OrderStatusMessage {
    pub fn new() -> Self {
        OrderStatusMessage {
            order_id: 0,
            status: "".to_string(),
            filled: 0.0,
            remaining: 0.0,
            avg_fill_price: 0.0,
            perm_id: 0,
            parent_id: 0,
            last_fill_price: 0.0,
            client_id: 0,
            why_held: "".to_string(),
            mkt_cap_price: 0.0
        }
    }
}