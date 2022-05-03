use crate::models::contract_details::ContractDetails;

#[derive(Debug)]
pub struct ScanDataItem {
    pub request_id: i32,
    pub rank: i32,
    pub contract_details: ContractDetails,
    pub distance: String,
    pub benchmark: String,
    pub projection: String,
    pub legs_str: String
}