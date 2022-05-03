#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum OrderConditionType {
    Price = 1,
    Time = 3,
    Margin = 4,
    Execution = 5,
    Volume = 6,
    PercentChange = 7,
    None,
}

impl From<i32> for OrderConditionType {
    fn from(val: i32) -> Self {
        return match val {
            1 => OrderConditionType::Price,
            3 => OrderConditionType::Time,
            4 => OrderConditionType::Margin,
            5 => OrderConditionType::Execution,
            6 => OrderConditionType::Volume,
            7 => OrderConditionType::PercentChange,
            _ => OrderConditionType::None
        }
    }
}
