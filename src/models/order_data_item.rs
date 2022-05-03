use crate::models::order::Order;
use crate::models::contract::Contract;
use crate::models::order_state::OrderState;

pub struct OrderDataItem {
    pub order_id: i32,
    pub order: Order,
    pub contract: Contract,
    pub order_state: OrderState
}

impl OrderDataItem {
    pub(crate) fn new(order_id: i32, order: Order, contract: Contract, order_state: OrderState) -> Self {
        OrderDataItem {
            order_id,
            order,
            contract,
            order_state
        }
    }
}
