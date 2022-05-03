#[derive(Debug)]
pub struct AccountSummary {
    pub request_id: i32,
    pub account: String,
    pub tag: String,
    pub value: String,
    pub currency: String
}
