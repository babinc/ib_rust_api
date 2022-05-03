use crate::enums::order_condition_type::OrderConditionType;
use crate::traits::order_condition::OrderCondition;
use crate::api_parameter::ApiParameters;
use std::slice::Iter;

#[derive(Debug)]
pub struct MarginCondition {
    pub is_conjunction_connection: bool,
    pub condition_type: OrderConditionType,
    pub percent: i32
}

impl MarginCondition {
    pub fn new() -> Self {
        MarginCondition {
            is_conjunction_connection: false,
            condition_type: OrderConditionType::Margin,

            percent: 0,
        }
    }
}

impl OrderCondition for MarginCondition {
    fn get_type(&self) -> i32 { self.condition_type as i32 }

    fn get_conjunction(&self) -> bool { self.is_conjunction_connection }

    fn set_conjunction(&mut self, val: bool) { self.is_conjunction_connection = val}

    fn serialize(&mut self, out_stream: &mut ApiParameters) {
        self.serialize_conjunction(out_stream);
    }

    fn deserialize(&mut self, _in_stream: &mut Iter<String>) {
        unimplemented!()
    }

    fn try_parse(&self, _cond: String) -> bool {
        unimplemented!()
    }

    fn get_hash_code(&self) -> i32 {
        unimplemented!()
    }
}
