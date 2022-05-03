use crate::enums::position_type::PositionType;

#[derive(Debug)]
pub struct ComboLeg {
    pub con_id: i32,
    pub ratio: i32,
    pub action: String,
    pub exchange: String,
    pub open_close: PositionType,
    pub short_sale_slot: i32,
    pub designated_location: String,
    pub exempt_code: i32,
}

impl ComboLeg {
    pub fn new(con_id: i32,
               ratio: i32,
               action: String,
               exchange: String,
               open_close: PositionType,
               short_sale_slot: i32,
               designated_location: String,
               exempt_code: i32) -> ComboLeg {
        ComboLeg {
            con_id,
            ratio,
            action,
            exchange,
            open_close,
            short_sale_slot,
            designated_location,
            exempt_code
        }
    }
}
