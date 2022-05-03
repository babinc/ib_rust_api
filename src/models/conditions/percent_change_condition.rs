use crate::enums::order_condition_type::OrderConditionType;
use crate::traits::order_condition::OrderCondition;
use crate::api_parameter::ApiParameters;
use crate::decoder::{decode_f64};
use std::slice::Iter;

#[derive(Debug)]
pub struct PercentChangeCondition {
    pub is_conjunction_connection: bool,
    pub condition_type: OrderConditionType,

    pub change_percent: f64,
}

impl PercentChangeCondition {
    pub fn new() -> Self {
        PercentChangeCondition {
            is_conjunction_connection: false,
            condition_type: OrderConditionType::PercentChange,

            change_percent: 0.0
        }
    }
}

impl OrderCondition for PercentChangeCondition {
    fn get_type(&self) -> i32 {
        self.condition_type as i32
    }

    fn get_conjunction(&self) -> bool { self.is_conjunction_connection }

    fn set_conjunction(&mut self, val: bool) { self.is_conjunction_connection = val}

    fn serialize(&mut self, out_stream: &mut ApiParameters) {
        self.serialize_conjunction(out_stream);

        out_stream.add_double(self.change_percent);
    }

    fn deserialize(&mut self, in_stream: &mut Iter<String>) {
        self.deserialize_conjunction(in_stream);

        self.change_percent = decode_f64(in_stream).unwrap_or_default();
    }

    fn try_parse(&self, _cond: String) -> bool {
        unimplemented!()
    }

    fn get_hash_code(&self) -> i32 {
        unimplemented!()
    }
}
