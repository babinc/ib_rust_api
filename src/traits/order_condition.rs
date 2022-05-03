use crate::api_parameter::ApiParameters;
use crate::decoder::{decode_string};
use std::slice::Iter;

pub trait OrderCondition: Send {
    fn get_type(&self) -> i32;
    fn get_conjunction(&self) -> bool;
    fn set_conjunction(&mut self, val: bool);
    fn serialize_conjunction(&self, out_stream: &mut ApiParameters) {
        match self.get_conjunction() {
            true => out_stream.add_string("a"),
            false => out_stream.add_string("o")
        }
    }
    fn deserialize_conjunction(&mut self, in_stream: &mut Iter<String>) {
        let val = decode_string(in_stream).unwrap_or_default() == "a".to_string();
        self.set_conjunction(val);
    }
    fn serialize(&mut self, out_stream: &mut ApiParameters);
    fn deserialize(&mut self, in_stream: &mut Iter<String>);
    fn try_parse(&self, cond: String) -> bool;
//    fn parse(&mut self, cond: String) -> Self;
//    fn equals(&self) -> bool;
    fn get_hash_code(&self) -> i32;
}
