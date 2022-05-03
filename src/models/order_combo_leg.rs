#[derive(Debug)]
pub struct OrderComboLeg {
    pub price: f64,
}

impl OrderComboLeg {
    pub fn new(price: f64) -> OrderComboLeg {
        OrderComboLeg {
            price
        }
    }
}