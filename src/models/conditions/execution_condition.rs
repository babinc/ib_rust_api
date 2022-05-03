use crate::enums::order_condition_type::OrderConditionType;
use crate::traits::order_condition::OrderCondition;
use crate::api_parameter::ApiParameters;
use crate::decoder::{decode_string};
use std::slice::Iter;

#[derive(Debug)]
pub struct ExecutionCondition {
    pub is_conjunction_connection: bool,
    pub condition_type: OrderConditionType,

    pub sec_type: String,
    pub exchange: String,
    pub symbol: String,
}

impl ExecutionCondition {
    pub fn new() -> Self {
        ExecutionCondition {
            is_conjunction_connection: false,
            condition_type: OrderConditionType::Execution,

            sec_type: "".to_string(),
            exchange: "".to_string(),
            symbol: "".to_string(),
        }
    }
}

impl OrderCondition for ExecutionCondition {
    fn get_type(&self) -> i32 {
        self.condition_type as i32
    }

    fn get_conjunction(&self) -> bool { self.is_conjunction_connection }

    fn set_conjunction(&mut self, val: bool) { self.is_conjunction_connection = val}

    fn serialize(&mut self, out_stream: &mut ApiParameters) {
        self.serialize_conjunction(out_stream);

        out_stream.add_string(self.sec_type.as_str());
        out_stream.add_string(self.exchange.as_str());
        out_stream.add_string(self.symbol.as_str());
    }

    fn deserialize(&mut self, in_stream: &mut Iter<String>) {
        self.deserialize_conjunction(in_stream);

        self.sec_type = decode_string(in_stream).unwrap_or_default();
        self.exchange = decode_string(in_stream).unwrap_or_default();
        self.symbol = decode_string(in_stream).unwrap_or_default();
    }

    fn try_parse(&self, _cond: String) -> bool {
        unimplemented!()
    }

    fn get_hash_code(&self) -> i32 {
        unimplemented!()
    }
}

