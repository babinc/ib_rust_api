#[derive(Debug)]
pub struct SoftDollarTier {
    pub name: String,
    pub val: String,
    pub display_name: String,
}

impl SoftDollarTier {
    pub fn new(name: String, val: String, display_name: String) -> SoftDollarTier {
       SoftDollarTier {
           name,
           val,
           display_name
       }
    }
}