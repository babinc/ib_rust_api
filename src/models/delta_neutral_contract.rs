#[derive(Debug)]
pub struct DeltaNeutralContract {
    pub con_id: i32,
    pub delta: f64,
    pub price: f64,
}

impl DeltaNeutralContract {
    pub fn new(con_id: i32, delta: f64, price: f64) -> DeltaNeutralContract {
        DeltaNeutralContract {
            con_id,
            delta,
            price
        }
    }
}