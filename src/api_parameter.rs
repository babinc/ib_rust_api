use std::io::{Write, Cursor};
use num_traits::real::Real;
use crate::constants::helper_constants;
use crate::models::tag_value::TagValue;

pub struct ApiParameters {
    pub cursor: Cursor<Vec<u8>>
}

impl ApiParameters {
    pub fn new() -> ApiParameters {
        ApiParameters {
            cursor: Cursor::new(Vec::new())
        }
    }

    pub fn add_string(&mut self, item: &str) {
        let bytes: &[u8] = item.as_bytes();
        self.cursor.write(bytes).unwrap_or_else(|err| {
            eprintln!("ib_rust_api add_string (1) error: {}", err.to_string());
            0
        });

        let eol = helper_constants::EOL.to_be_bytes();
        self.cursor.write(&eol).unwrap_or_else(|err| {
            eprintln!("ib_rust_api add_string (2) error: {}", err.to_string());
            0
        });
    }

    pub fn add_string_without_eol(&mut self, item: &str) {
        let bytes: &[u8] = item.as_bytes();
        self.cursor.write(bytes).unwrap_or_else(|err| {
            eprintln!("ib_rust_api add_string_without_eol error: {}", err.to_string());
            0
        });
    }

    pub fn add_int(&mut self, item: i32) {
        let bytes = item.to_string();
        self.add_string(bytes.as_str());
    }

    pub fn add_double(&mut self, item: f64) {
        let str_item = item.to_string();
        self.add_string(str_item.as_str());
    }

    pub fn add_int_max(&mut self, item: i32) {
        if item == i32::max_value() {
            let eol = helper_constants::EOL.to_be_bytes();
            self.cursor.write(&eol).unwrap_or_else(|err| {
                eprintln!("ib_rust_api add_int_max error: {}", err.to_string());
                0
            });
        }
        else {
            self.add_int(item);
        }
    }

    pub fn add_double_max(&mut self, item: f64) {
        if item == f64::max_value() {
            let eol = helper_constants::EOL.to_be_bytes();
            self.cursor.write(&eol).unwrap_or_else(|err| {
                eprintln!("ib_rust_api add_double_max error: {}", err.to_string());
                0
            });
        }
        else {
            self.add_double(item);
        }
    }

    pub fn add_bool(&mut self, item: bool) {
        let write_value;
        if item {
            write_value = "1"
        }
        else {
            write_value = "0"
        }
        self.add_string(write_value);
    }

    pub fn prepare_buffer(&mut self, use_v100_plus: bool) -> u32 {
        let rval = self.cursor.get_ref().len();

        if use_v100_plus {
            let val = 0_i32.to_be_bytes();
            self.cursor.write(&val).unwrap_or_else(|err| {
                eprintln!("ib_rust_api prepare_buffer error: {}", err.to_string());
                0
            });
        }

        rval as u32
    }

    pub fn add_tag_value_vec(&mut self, items: Vec<TagValue>) {
        let mut tag_values = "".to_string();

        for item in items.iter() {
            tag_values += format!("{}={};", item.tag, item.value).as_ref();
        }

        self.add_string(tag_values.as_ref());
    }
}

