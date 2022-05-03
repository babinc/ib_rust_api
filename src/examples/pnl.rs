use std::error::Error;
use ib_rust_api::enums::incoming_message_enum::IncomingMessagesEnum;
use ib_rust_api::IbClient;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running real time bars example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    ib_client.req_pnl(123, "DU1650630", "");

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::PnL(req_id, daily, unrealized, realized) => {
                println!("Req Id: {}, daily: {}, realized {}, unrealized {}", req_id, daily, realized, unrealized);
            },
            _ => {
                println!("Event not handled");
            }
        }
    }
}

