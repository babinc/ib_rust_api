use std::error::Error;
use ib_rust_api::IbClient;
use ib_rust_api::enums::incoming_message_enum::IncomingMessagesEnum;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running live quotes example");

    let mut ib_client = IbClient::new("127.0.0.1".to_string(), 7497, 3);
    ib_client.connect()?;

    ib_client.req_account_summary(6);

    loop {
        let event = ib_client.event_receiver.recv().unwrap();
        match event {
            IncomingMessagesEnum::Error(id, code, msg) => {
                println!("ERR: id: {}, code: {}, msg: {}", id, code, msg);
            },
            IncomingMessagesEnum::ManagedAccounts(accounts) => {
                println!("Managed Accounts: {}", accounts);
            },
            IncomingMessagesEnum::AccountSummary(account_summary) => {
                println!("Account Summary: Account, {}, tag, {}, value, {}, currency, {}", account_summary.account, account_summary.tag, account_summary.value, account_summary.currency);
            },
            IncomingMessagesEnum::AccountSummaryEnd(_) => {
                println!("Account summary end");
            }
            _ => {
                println!("Event not handled");
            }
        }
    }
}

