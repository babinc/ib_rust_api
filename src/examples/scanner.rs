use std::error::Error;
use std::io::stdin;
use ib_rust_api::IbClient;
use ib_rust_api::enums::incoming_message_enum::IncomingMessagesEnum;
use ib_rust_api::models::scanner_subscription::ScannerSubscription;

const REQ_ID: i32 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 2);

    ib_client.connect()?;

    scanner_request(&mut ib_client);

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::ScannerData(val) => {
                println!();
                for item in val {
                    println!("Scan Data: {}, {}", item.contract_details.contract.symbol, item.contract_details.contract.exchange);
                }

                break;
            },
            _ => {
                println!("Event not handled");
            }
        }
    }

    println!("Canceling subscription");
    ib_client.cancel_scanner_subscription(REQ_ID);

    read_line()?;

    Ok(())
}

fn scanner_request(ib_client: &mut IbClient) {
    let mut scanner_subscription = ScannerSubscription::new();
    scanner_subscription.number_of_rows = 15;
    scanner_subscription.instrument = "STK".to_string();
    scanner_subscription.location_code = "STK.NASDAQ".to_string();
    scanner_subscription.scan_code = "TOP_PERC_GAIN".to_string();
    scanner_subscription.stock_type_filter = "ALL".to_string();
    scanner_subscription.above_price = 1.0;
    scanner_subscription.below_price = 25.0;
    scanner_subscription.market_cap_above = 100_000.0;
    scanner_subscription.market_cap_below = 100_000_000.0;
    scanner_subscription.above_volume = 200_000;
    ib_client.req_scanner_subscription(REQ_ID, scanner_subscription);
}

fn read_line() -> Result<usize, std::io::Error> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
}
