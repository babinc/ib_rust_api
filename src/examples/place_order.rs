use IbRustApi::models::contract::Contract;
use std::error::Error;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;
use IbRustApi::IbClient;
use IbRustApi::models::order::Order;
use std::thread;
use std::sync::{Arc, Mutex};
use rsevents::{AutoResetEvent, State, Awaitable};
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running place order example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::NextValidId(val) => {
                let mut contract = Contract::new();
                contract.symbol = "IBM".to_string();
                contract.sec_type = "STK".to_string();
                contract.exchange = "SMART".to_string();
                contract.currency = "USD".to_string();

                let mut order = Order::new();
                order.action = "BUY".to_string();
                order.lmt_price = 124.0;
                order.order_type = "LMT".to_string();
                order.order_id = val;
                order.total_quantity = 20.0;
                order.tif = "DAY".to_string();
                order.volatility_type = 0;
                order.reference_price_type = 0;

                ib_client.place_order(val, contract, order);
            },
            IncomingMessagesEnum::OrderStatus(order_status_message) => {
                println!("Status: {}, filled: {}", order_status_message.status, order_status_message.filled);
            },
            _ => {
                println!("Event not handled");
            }
        }
    }

    Ok(())
}
