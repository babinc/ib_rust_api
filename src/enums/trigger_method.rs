#[derive(Debug, Clone, Copy)]
pub enum TriggerMethod {
    Default = 0,
    DoubleBidAsk = 1,
    Last = 2,
    DoubleLast = 3,
    BidAsk = 4,
    NA1 = 5,
    NA2 = 6,
    LastBidAsk = 7,
    MidPoint = 8,
}

impl From<i32> for TriggerMethod {
    fn from(val: i32) -> Self {
        match val {
            0 => TriggerMethod::Default,
            1 => TriggerMethod::DoubleBidAsk,
            2 => TriggerMethod::Last,
            3 => TriggerMethod::DoubleLast,
            4 => TriggerMethod::BidAsk,
            5 => TriggerMethod::NA1,
            6 => TriggerMethod::NA2,
            7 => TriggerMethod::LastBidAsk,
            8 => TriggerMethod::MidPoint,
            _ => TriggerMethod::Default
        }
    }
}