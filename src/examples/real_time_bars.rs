use IbRustApi::models::contract::Contract;
use std::error::Error;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;
use IbRustApi::IbClient;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running real time bars example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    let mut contract = Contract::new();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    contract.sec_type = "STK".to_string();
    contract.symbol = "AAPL".to_string();
    let what_to_show = "TRADES";
    ib_client.req_real_time_bars(12, contract, 5, what_to_show, false, vec![]);

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::RealTimeBars(req_id, bar) => {
                println!("time int: {}, time str: {}, Volume {}, Close {}", bar.time_int, bar.time_str, bar.volume, bar.close)
            },
            _ => {
                println!("Event not handled");
            }
        }
    }
}

