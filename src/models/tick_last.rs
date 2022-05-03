#[derive(Debug, Clone)]
pub struct TickLast {
    pub time: i64,
    pub price: f64,
    pub size: i32,
    pub mask: i32,
    pub exchange: String,
    pub special_conditions: String
}

impl TickLast {
    pub fn new() -> Self {
        TickLast {
            time: 0,
            price: 0.0,
            size: 0,
            mask: 0,
            exchange: "".to_string(),
            special_conditions: "".to_string()
        }
    }
}