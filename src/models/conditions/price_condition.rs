use crate::enums::trigger_method::TriggerMethod;
use crate::traits::order_condition::OrderCondition;
use crate::api_parameter::ApiParameters;
use crate::decoder::{decode_i32};
use crate::enums::order_condition_type::OrderConditionType;
use std::slice::Iter;

#[derive(Debug)]
pub struct PriceCondition {
    pub is_conjunction_connection: bool,
    pub condition_type: OrderConditionType,

    pub price: f64,
    pub trigger_method: TriggerMethod,
}

impl PriceCondition {
    pub fn new() -> Self {
        PriceCondition {
            is_conjunction_connection: false,
            condition_type: OrderConditionType::Price,

            price: 0.0,
            trigger_method: TriggerMethod::Default
        }
    }
}

impl OrderCondition for PriceCondition {
    fn get_type(&self) -> i32 { self.condition_type as i32 }

    fn get_conjunction(&self) -> bool { self.is_conjunction_connection }

    fn set_conjunction(&mut self, val: bool) { self.is_conjunction_connection = val}

    fn serialize(&mut self, out_stream: &mut ApiParameters) {
        self.serialize_conjunction(out_stream);

        out_stream.add_int(self.trigger_method as i32);
    }

    fn deserialize(&mut self, in_stream: &mut Iter<String>) {
        self.deserialize_conjunction(in_stream);

        let num = decode_i32(in_stream).unwrap_or_default();
        self.trigger_method = TriggerMethod::from(num);
    }

    fn try_parse(&self, _cond: String) -> bool {
        unimplemented!()
    }

    fn get_hash_code(&self) -> i32 {
        unimplemented!()
    }
}