use std::error::Error;
use IbRustApi::IbClient;
use IbRustApi::models::contract::Contract;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;

const REQ_ID: i32 = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    let mut contract = Contract::new();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    contract.sec_type = "STK".to_string();
    contract.symbol = "APVO".to_string();

    ib_client.req_market_data(REQ_ID, &contract, "236", false, false, vec!());

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::TickGeneric(req_id, tick_id, value) => {
                println!("req_id: {}, tick id: {}, value: {}", req_id, tick_id, value);
            },
            _ => {
                println!("Event not handled");
            }
        }
    }

    Ok(())
}
