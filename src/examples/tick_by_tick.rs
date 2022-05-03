use std::error::Error;
use IbRustApi::IbClient;
use IbRustApi::models::contract::Contract;
use IbRustApi::enums::incoming_message_enum::IncomingMessagesEnum;
use IbRustApi::models::bar::{Bar, BarColors};
use chrono::{DateTime, Utc, NaiveDateTime, Duration, Timelike};
use IbRustApi::models::tick_last::TickLast;
use ib_rust_api::IbClient;
use ib_rust_api::models::contract::Contract;
use ib_rust_api::models::bar::{Bar, BarColors};
use ib_rust_api::models::tick_last::TickLast;
use ib_rust_api::enums::incoming_message_enum::IncomingMessagesEnum;

const REQ_ID: i32 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);
    ib_client.connect()?;

    let mut contract_1 = Contract::new();
    contract_1.currency = "USD".to_string();
    contract_1.exchange = "SMART".to_string();
    contract_1.sec_type = "STK".to_string();
    contract_1.symbol = "AAPL".to_string();
    ib_client.req_tick_by_tick(3, contract_1, "Last", 1, false);

    let mut contract_2 = Contract::new();
    contract_2.currency = "USD".to_string();
    contract_2.exchange = "SMART".to_string();
    contract_2.sec_type = "STK".to_string();
    contract_2.symbol = "INO".to_string();
    ib_client.req_tick_by_tick(4, contract_2, "Last", 1, false);

    let mut symbols: Vec<(i32, Bar, TickLast, u32)> = vec![];

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::TickByTickLast((req_id, tick)) => {
                let time = convert_from_unix(tick.time);
                let hour = time.hour();
                let minute = time.minute();
                let secs = time.second();

                // println!("tick id: {}- Time: {}:{}:{} - {:?}", req_id, hour, minute, secs, tick);

                aggregate_min_bar(&mut symbols, req_id, tick, hour, minute, secs)
            },
            _ => {
                println!("Event not handled");
            }
        }
    }

    Ok(())

}

fn aggregate_min_bar(symbols: &mut Vec<(i32, Bar, TickLast, u32)>, req_id: i32, tick: TickLast, hour: u32, minute: u32, secs: u32) {
    if symbols.iter_mut().find(|x| x.0 == req_id).is_none() {
        symbols.push((req_id, Bar::new(), TickLast::new(), 0_u32));
    }

    for symbol in symbols.iter_mut() {
        if symbol.0 == req_id {
            if symbol.1.open == 0.0 {
                symbol.1.open = tick.price;
            }
            if tick.price > symbol.1.high {
                symbol.1.high = tick.price;
            }
            if tick.price < symbol.1.low || symbol.1.low == 0.0 {
                symbol.1.low = tick.price;
            }

            if symbol.3 != minute {
                symbol.3 = minute;
                symbol.1.close = symbol.2.price;

                if symbol.1.close > symbol.1.open {
                    symbol.1.color = Some(BarColors::Green);
                }
                else if symbol.1.close < symbol.1.open {
                    symbol.1.color = Some(BarColors::Red);
                }
                else {
                    symbol.1.color = Some(BarColors::Yellow);
                }

                let minute_time_str = format!("{}:{}:{}", hour, (minute - 1), secs);
                symbol.1.time_str = minute_time_str;
                println!("req_id: {} - Minute bar: {:?}", req_id, symbol.1);

                symbol.1 = Bar::new();
            }
            symbol.2 = tick.clone();
        }
    }
}

pub fn convert_from_unix(unix_time: i64) -> DateTime<Utc> {
    let naive_datetime = NaiveDateTime::from_timestamp(unix_time, 0);
    let time: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc) - Duration::hours(5);
    time
}
