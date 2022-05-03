#[derive(Debug)]
pub struct TagValue {
    pub tag: String,
    pub value: String,
}

impl TagValue {
    pub fn new(tag: String, value: String) -> TagValue {
        TagValue {
            tag,
            value
        }
    }
}