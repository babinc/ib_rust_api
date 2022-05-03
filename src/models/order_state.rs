#[derive(Debug)]
pub struct OrderState {
    pub status: String,
    pub init_margin_before: String,
    pub maint_margin_before: String,
    pub equity_with_loan_before: String,
    pub init_margin_change: String,
    pub maint_margin_change: String,
    pub equity_with_loan_change: String,
    pub init_margin_after: String,
    pub maint_margin_after: String,
    pub equity_with_loan_after: String,
    pub commission: f64,
    pub min_commission: f64,
    pub max_commission: f64,
    pub commission_currency: String,
    pub warning_text: String,
    pub completed_time: String,
    pub completed_status: String,
}

impl OrderState {
    pub fn new() -> OrderState {
        OrderState {
            status: "".to_string(),
            init_margin_before: "".to_string(),
            maint_margin_before: "".to_string(),
            equity_with_loan_before: "".to_string(),
            init_margin_change: "".to_string(),
            maint_margin_change: "".to_string(),
            equity_with_loan_change: "".to_string(),
            init_margin_after: "".to_string(),
            maint_margin_after: "".to_string(),
            equity_with_loan_after: "".to_string(),
            commission: 0.0,
            min_commission: 0.0,
            max_commission: 0.0,
            commission_currency: "".to_string(),
            warning_text: "".to_string(),
            completed_time: "".to_string(),
            completed_status: "".to_string()
        }
    }
}