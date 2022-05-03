use std::error::Error;
use IbRustApi::IbClient;
use std::io::stdin;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;
use IbRustApi::models::contract::Contract;
use chrono::{Utc};

const REQ_ID: i32 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running historical data example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    let mut contract = Contract::new();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    contract.sec_type = "STK".to_string();
    contract.symbol = "AAPL".to_string();

    let dt = Utc::now();
    let query_time = dt.format("%Y%m%d %H:%M:%S").to_string();

    let duration = "2 D";
    let bar_size = "1 min";
    let what_to_show = "TRADES";

    ib_client.req_historical_data(REQ_ID, contract, query_time.as_str(), duration, bar_size, what_to_show, 0, 1, false, vec![]);

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::HistoricalData(bar) => {
                println!("time int: {}, time str: {}, Volume {}, Close {}", bar.time_int, bar.time_str, bar.volume, bar.close)
            },
            IncomingMessagesEnum::HistoricalDataEnd(req_id, start_date_time, end_date_time) => {
                println!("Historical Data End, req_id: {}, start time: {}, end time: {}", req_id, start_date_time, end_date_time);
                break
            },
            _ => {
                println!("Event not handled");
            }
        }
    }

    read_line();

    Ok(())
}

fn read_line() -> Result<usize, std::io::Error> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
}
