use IbRustApi::models::contract::Contract;
use std::error::Error;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;
use IbRustApi::IbClient;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running contract details example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    let mut contract = Contract::new();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    contract.sec_type = "STK".to_string();
    contract.symbol = "AAPL".to_string();
    ib_client.req_contract_details(13, &contract);

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::ContractData(req_id, contract_details) => {
                println!("Req Id: {}, Contract: min tick {}", req_id, contract_details.min_tick)
            },
            IncomingMessagesEnum::ContractDataEnd(req_id) => {
                println!("Req_id: {}. Done", req_id);
            },
            _ => {
                println!("Event not handled");
            }
        }
    }
}

