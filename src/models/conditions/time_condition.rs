use crate::traits::order_condition::OrderCondition;
use crate::api_parameter::ApiParameters;
use crate::decoder::{decode_string};
use crate::enums::order_condition_type::OrderConditionType;
use std::slice::Iter;

#[derive(Debug)]
pub struct TimeCondition {
    pub is_conjunction_connection: bool,
    pub condition_type: OrderConditionType,

    pub time: String,
}

impl TimeCondition {
    pub fn new() -> Self {
        TimeCondition {
            is_conjunction_connection: false,
            condition_type: OrderConditionType::Time,

            time: "".to_string()
        }
    }
}

impl OrderCondition for TimeCondition {
    fn get_type(&self) -> i32 {
        self.condition_type as i32
    }

    fn get_conjunction(&self) -> bool { self.is_conjunction_connection }

    fn set_conjunction(&mut self, val: bool) { self.is_conjunction_connection = val}

    fn serialize(&mut self, out_stream: &mut ApiParameters) {
        self.serialize_conjunction(out_stream);

        out_stream.add_string(self.time.as_str());
    }

    fn deserialize(&mut self, in_stream: &mut Iter<String>) {
        self.deserialize_conjunction(in_stream);

        self.time = decode_string(in_stream).unwrap_or_default();
    }

    fn try_parse(&self, _cond: String) -> bool {
        unimplemented!()
    }

    fn get_hash_code(&self) -> i32 {
        unimplemented!()
    }
}