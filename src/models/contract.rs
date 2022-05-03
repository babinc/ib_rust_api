use crate::models::combo_leg::ComboLeg;
use crate::models::delta_neutral_contract::DeltaNeutralContract;

#[derive(Debug)]
pub struct Contract {
    pub con_id: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: String,
    pub multiplier: String,
    pub exchange: String,
    pub primary_exchange: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub include_expired: bool,
    pub sec_id_type: String,
    pub sec_id: String,

    //combos
    pub combo_legs_descrip: String,
    pub combo_legs: Vec<ComboLeg>,
    pub delta_neutral_contract: Option<DeltaNeutralContract>,
}

impl Contract {
    pub fn new() -> Contract {
        Contract {
            con_id: 0,
            symbol: "".to_string(),
            sec_type: "".to_string(),
            last_trade_date_or_contract_month: "".to_string(),
            strike: 0.0,
            right: "".to_string(),
            multiplier: "".to_string(),
            exchange: "".to_string(),
            primary_exchange: "".to_string(),
            currency: "".to_string(),
            local_symbol: "".to_string(),
            trading_class: "".to_string(),
            include_expired: false,
            sec_id_type: "".to_string(),
            sec_id: "".to_string(),
            combo_legs_descrip: "".to_string(),
            combo_legs: vec![],
            delta_neutral_contract: None
        }
    }
}