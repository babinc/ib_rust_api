extern crate num;
#[macro_use]
extern crate num_derive;
extern crate rsevents;
extern crate regex;

use crate::client_socket::ClientSocket;
use std::string::ToString;
use crossbeam_channel::{unbounded, Receiver, Sender};
use crate::api_parameter::ApiParameters;
use crate::models::scanner_subscription::ScannerSubscription;
use crate::enums::incoming_message_enum::IncomingMessagesEnum;
use crate::enums::outgoing_messages::OutgoingMessages;
use crate::models::contract::Contract;
use crate::models::order::Order;
use crate::errors::client_errors;
use crate::models::tag_value::TagValue;
use crate::constants::{min_server_version, helper_constants};
use std::error::Error;

mod client_socket;
mod api_parameter;
mod decoder;
pub mod models;
mod constants;
pub mod enums;
mod order_decoder;
mod traits;
mod errors;

pub struct IbClient {
    pub event_receiver: Receiver<IncomingMessagesEnum>,
    event_sender: Sender<IncomingMessagesEnum>,
    client_socket: ClientSocket,
    is_connected: bool,
}

impl IbClient {
    pub fn new(host: String, port: i32, client_id: i32) -> IbClient {
        let (event_sender, event_receiver) = unbounded();

        let e_client_socket = ClientSocket::new(host, port, client_id, event_sender.clone());

        let ib_client = IbClient {
            client_socket: e_client_socket,
            event_receiver,
            event_sender,
            is_connected: false,
        };

        ib_client
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        match self.client_socket.connect() {
            Ok(_) => {
                self.is_connected = true;
            },
            Err(e) => {
                self.is_connected = false;
                return Err(e);
            },
        }

        Ok(())
    }

    pub fn req_market_data(&mut self, req_id: i32, contract: &Contract, generic_tick_list: &str, snapshot: bool, regulatory_snapshot: bool, market_data_options: Vec<TagValue>) {
        if self.check_connection() == false {
            return;
        }

        if snapshot && self.check_server_version(req_id, min_server_version::SNAPSHOT_MKT_DATA, "It does not support snapshot market data") == false {
            return;
        }

        if let Some(_val) = &contract.delta_neutral_contract {
            if self.check_server_version(req_id, min_server_version::DELTA_NEUTRAL, "It does not support delta-neutral orders") == false {
                return;
            }
        }

        if contract.con_id > 0 && self.check_server_version(req_id, min_server_version::CONTRACT_CONID, "It does not support con_id parameter") == false {
            return;
        }

        if contract.trading_class.is_empty() && self.check_server_version(req_id, min_server_version::TRADING_CLASS, "It does not support trading class parameter in req_market_data") == false {
            return;
        }

        let version = 11;

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestMarketData as i32);
        params_list.add_int(version);
        params_list.add_int(req_id);

        if self.client_socket.server_version >= min_server_version::CONTRACT_CONID
        {
            params_list.add_int(contract.con_id);
        }

        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());

        if self.client_socket.server_version >= 15 {
            params_list.add_string(contract.multiplier.as_str());
        }

        params_list.add_string(contract.exchange.as_str());

        if self.client_socket.server_version >= 14 {
            params_list.add_string(contract.primary_exchange.as_str());
        }

        params_list.add_string(contract.currency.as_str());

        if self.client_socket.server_version >= 2 {
            params_list.add_string(contract.local_symbol.as_str());
        }

        if self.client_socket.server_version >= min_server_version::TRADING_CLASS {
            params_list.add_string(contract.trading_class.as_str());
        }


        if self.client_socket.server_version >= 8 && IbClient::strings_are_equal(helper_constants::BAG_SEC_TYPE, contract.sec_type.as_str()) {
            if contract.combo_legs.len() == 0 {
                params_list.add_int(0);
            }
            else {
                params_list.add_int(contract.combo_legs.len() as i32);
                for combo_leg in contract.combo_legs.iter() {
                    params_list.add_int(combo_leg.con_id);
                    params_list.add_int(combo_leg.ratio);
                    params_list.add_string(combo_leg.action.as_ref());
                    params_list.add_string(combo_leg.exchange.as_ref());
                }
            }
        }

        if self.client_socket.server_version >= min_server_version::DELTA_NEUTRAL {
            match &contract.delta_neutral_contract {
                Some(delta_neutral_contract) => {
                    params_list.add_bool(true);
                    params_list.add_int(delta_neutral_contract.con_id);
                    params_list.add_double(delta_neutral_contract.delta);
                    params_list.add_double(delta_neutral_contract.price);
                },
                None => {
                    params_list.add_bool(false);
                }
            }
        }

        if self.client_socket.server_version >= 31 {
            params_list.add_string(generic_tick_list);
        }

        if self.client_socket.server_version >= min_server_version::SNAPSHOT_MKT_DATA {
            params_list.add_bool(snapshot);
        }

        if self.client_socket.server_version >= min_server_version::SMART_COMPONENTS {
            params_list.add_bool(regulatory_snapshot);
        }

        if self.client_socket.server_version >= min_server_version::LINKING {
            params_list.add_tag_value_vec(market_data_options);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQMKT, err.to_string().as_ref());
        });
    }

    pub fn req_scanner_subscription(&mut self, req_id: i32, subscription: ScannerSubscription) {
        if self.check_connection() == false {
            return;
        }

        const VERSION: i32 = 4;
        let server_version = self.client_socket.server_version;
        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestScannerSubscription as i32);

        if server_version < min_server_version::SCANNER_GENERIC_OPTS {
            params_list.add_int(VERSION);
        }

        params_list.add_int(req_id);
        params_list.add_int_max(subscription.number_of_rows);
        params_list.add_string(subscription.instrument.as_str());
        params_list.add_string(subscription.location_code.as_str());
        params_list.add_string(subscription.scan_code.as_str());

        params_list.add_double_max(subscription.above_price);
        params_list.add_double_max(subscription.below_price);
        params_list.add_int_max(subscription.above_volume);
        params_list.add_double_max(subscription.market_cap_above);
        params_list.add_double_max(subscription.market_cap_below);
        params_list.add_string(subscription.moody_rating_above.as_str());
        params_list.add_string(subscription.moody_rating_below.as_str());
        params_list.add_string(subscription.sp_rating_above.as_str());
        params_list.add_string(subscription.sp_rating_below.as_str());
        params_list.add_string(subscription.maturity_date_above.as_str());
        params_list.add_string(subscription.maturity_date_below.as_str());
        params_list.add_double_max(subscription.coupon_rate_above);
        params_list.add_double_max(subscription.coupon_rate_below);
        params_list.add_bool(subscription.exclude_convertible);

        if server_version >= 25
        {
            params_list.add_int_max(subscription.average_option_volume_above);
            params_list.add_string(subscription.scanner_setting_pairs.as_str());
        }

        if server_version >= 27
        {
            params_list.add_string(subscription.stock_type_filter.as_str());
        }

        if server_version >= min_server_version::SCANNER_GENERIC_OPTS
        {
            //TODO
            let scanner_subscription_filter_options = "";
            params_list.add_string(scanner_subscription_filter_options);
        }

        if server_version >= min_server_version::LINKING
        {
            //TODO
            let scanner_subscription_options = "";
            params_list.add_string(scanner_subscription_options);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQMKT, err.to_string().as_ref());
        });
    }

    pub fn req_account_summary(&mut self, req_id: i32) {
        if self.check_connection() == false {
            return;
        }

        let group = "All";
        let tags = "AccountType,NetLiquidation,TotalCashValue,SettledCash,AccruedCash,BuyingPower,EquityWithLoanValue,PreviousEquityWithLoanValue,GrossPositionValue,ReqTEquity,ReqTMargin,SMA,InitMarginReq,MaintMarginReq,AvailableFunds,ExcessLiquidity,Cushion,FullInitMarginReq,FullMaintMarginReq,FullAvailableFunds,FullExcessLiquidity,LookAheadNextChange,LookAheadInitMarginReq ,LookAheadMaintMarginReq,LookAheadAvailableFunds,LookAheadExcessLiquidity,HighestSeverity,DayTradesRemaining,Leverage";

        const VERSION: i32 = 1;
        if self.client_socket.is_connected == false {
            return
        }

        if self.client_socket.server_version < min_server_version::ACCT_SUMMARY {
            eprintln!("It does not support account summary request");
            return;
        }

        let mut param_list = ApiParameters::new();
        let length_pos = param_list.prepare_buffer(self.client_socket.use_v1000_plus);

        param_list.add_int(OutgoingMessages::RequestAccountSummary as i32);
        param_list.add_int(VERSION);
        param_list.add_int(req_id);
        param_list.add_string(group);
        param_list.add_string(tags);

        self.client_socket.close_and_send(&mut param_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQMKT, err.to_string().as_ref());
        });
    }

    pub fn req_contract_details(&mut self, req_id: i32, contract: &Contract) {
        if self.check_connection() == false {
            return;
        }

        let server_version = self.client_socket.server_version;
        if contract.sec_id_type.is_empty() == false || contract.sec_id.is_empty() == false {
            if self.check_server_version(req_id, min_server_version::SEC_ID_TYPE, "It does not support secIdType not secId attributes") == false {
                return;
            }
        }

        if contract.trading_class.is_empty() == false {
            if self.check_server_version(req_id, min_server_version::TRADING_CLASS, "It does not support the TradingClass parameter when requesting contract details.") == false {
                return;
            }
        }

        if contract.primary_exchange.is_empty() == false &&
            self.check_server_version(req_id, min_server_version::LINKING, " It does not support PrimaryExch parameter when requesting contract details.") == false {
            return;
        }

        const VERSION: i32 = 8;

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestContractData as i32);
        params_list.add_int(VERSION);
        if server_version >= min_server_version::CONTRACT_DATA_CHAIN
        {
            params_list.add_int(req_id);
        }
        if server_version >= min_server_version::CONTRACT_CONID
        {
            params_list.add_int(contract.con_id);
        }
        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());
        if server_version >= 15
        {
            params_list.add_string(contract.multiplier.as_str());
        }

        if server_version >= min_server_version::PRIMARYEXCH
        {
            params_list.add_string(contract.exchange.as_str());
            params_list.add_string(contract.primary_exchange.as_str());
        }
        else if server_version >= min_server_version::LINKING
        {
            if contract.primary_exchange.is_empty() == false && (contract.exchange == "BEST".to_string() || contract.exchange == "SMART".to_string())
            {
                params_list.add_string(format!("{}:{}", contract.exchange, contract.primary_exchange).as_str());
            }
            else
            {
                params_list.add_string(contract.exchange.as_str());
            }
        }

        params_list.add_string(contract.currency.as_str());
        params_list.add_string(contract.local_symbol.as_str());
        if server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_string(contract.trading_class.as_str());
        }
        if server_version >= 31
        {
            params_list.add_bool(contract.include_expired);
        }
        if server_version >= min_server_version::SEC_ID_TYPE
        {
            params_list.add_string(contract.sec_id_type.as_str());
            params_list.add_string(contract.sec_id.as_str());
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQCONTRACT, err.to_string().as_str());
        });
    }

    pub fn req_global_cancel(&mut self) {
        if self.check_connection() == false {
            return;
        }

        let req_id = 4;

        if self.check_server_version(7, min_server_version::REQ_GLOBAL_CANCEL, "It does not support global cancel requests.") == false {
            return;
        }

        const VERSION: i32 = 1;

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestGlobalCancel as i32);
        params_list.add_int(VERSION);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQGLOBALCANCEL, err.to_string().as_str());
        });
    }

    pub fn req_cancel_order(&mut self, req_id: i32, order_id: i32) {
        if self.check_connection() == false {
            return;
        }

        const VERSION: i32 = 2;

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::CancelOrder as i32);
        params_list.add_int(VERSION);
        params_list.add_int(order_id);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQGLOBALCANCEL, err.to_string().as_str());
        });
    }

    pub fn req_historical_data(&mut self, req_id: i32, contract: Contract, end_date_time: &str, duration: &str, bar_size_setting: &str, what_to_show: &str, use_rth: i32, date_format: i32, keep_up_to_date: bool, chart_options: Vec<TagValue>) {
        let server_version = self.client_socket.server_version;

        if self.check_connection() == false {
            return;
        }

        if self.check_server_version(req_id, 16, "") == false {
            return;
        }

        if contract.trading_class.is_empty() == false || contract.con_id > 0
        {
            if self.check_server_version(req_id, min_server_version::TRADING_CLASS, "It does not support conId nor trading class parameters when requesting historical data.") == false {
                return;
            }
        }

        const VERSION: i32 = 6;
        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestHistoricalData as i32);

        if server_version < min_server_version::SYNT_REALTIME_BARS
        {
            params_list.add_int(VERSION);
        }

        params_list.add_int(req_id);

        if server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_int(contract.con_id);
        }

        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());
        params_list.add_string(contract.multiplier.as_str());
        params_list.add_string(contract.exchange.as_str());
        params_list.add_string(contract.primary_exchange.as_str());
        params_list.add_string(contract.currency.as_str());
        params_list.add_string(contract.local_symbol.as_str());

        if server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_string(contract.trading_class.as_str());
        }

        let include_expired = if contract.include_expired { 1 } else { 0 };
        params_list.add_int(include_expired);

        params_list.add_string(end_date_time);
        params_list.add_string(bar_size_setting);

        params_list.add_string(duration);
        params_list.add_int(use_rth);
        params_list.add_string(what_to_show);

        params_list.add_int(date_format);

        if IbClient::strings_are_equal(helper_constants::BAG_SEC_TYPE, contract.sec_type.as_str())
        {
            if contract.combo_legs.len() == 0
            {
                params_list.add_int(0);
            }
            else
            {
                params_list.add_int(contract.combo_legs.len() as i32);

                for combo_leg in contract.combo_legs {
                    params_list.add_int(combo_leg.con_id);
                    params_list.add_int(combo_leg.ratio);
                    params_list.add_string(combo_leg.action.as_str());
                    params_list.add_string(combo_leg.exchange.as_str());
                }
            }
        }

        if server_version >= min_server_version::SYNT_REALTIME_BARS
        {
            params_list.add_bool(keep_up_to_date);
        }

        if server_version >= min_server_version::LINKING
        {
            params_list.add_tag_value_vec(chart_options);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQHISTDATA, err.to_string().as_str());
        });
    }

    pub fn req_real_time_bars(&mut self, req_id: i32, contract: Contract, bar_size: i32, what_to_show: &str, use_rth: bool, real_time_bar_options: Vec<TagValue>) {
        let server_version = self.client_socket.server_version;

        if self.check_connection() == false {
            return;
        }

        if self.check_server_version(req_id, min_server_version::REAL_TIME_BARS, "It does not support real time bars.") == false {
            return;
        }

        if contract.trading_class.is_empty() == false || contract.con_id > 0 {
            if self.check_server_version(req_id, min_server_version::TRADING_CLASS, "It does not support ConId nor TradingClass parameters in reqRealTimeBars.") == false {
                return;
            }
        }

        const VERSION: i32 = 3;
        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::RequestRealTimeBars as i32);
        params_list.add_int(VERSION);
        params_list.add_int(req_id);

        // params_list.AddParameter contract fields
        if server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_int(contract.con_id);
        }

        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());
        params_list.add_string(contract.multiplier.as_str());
        params_list.add_string(contract.exchange.as_str());
        params_list.add_string(contract.primary_exchange.as_str());
        params_list.add_string(contract.currency.as_str());
        params_list.add_string(contract.local_symbol.as_str());

        if server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_string(contract.trading_class.as_str());
        }

        params_list.add_int(bar_size);  // this parameter is not currently used
        params_list.add_string(what_to_show);
        params_list.add_bool(use_rth);

        if server_version >= min_server_version::LINKING
        {
            params_list.add_tag_value_vec(real_time_bar_options);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQRTBARS, err.to_string().as_str());
        });
    }

    pub fn req_tick_by_tick(&mut self, req_id: i32, contract: Contract, tick_type: &str, number_of_ticks: i32, ignore_size: bool) {
        if self.check_connection() == false {
            return;
        }

        if self.check_server_version(req_id, min_server_version::TICK_BY_TICK, "It does not support tick-by-tick request") == false {
            return;
        }

        if (number_of_ticks != 0 || ignore_size) && self.check_server_version(req_id, min_server_version::TICK_BY_TICK_IGNORE_SIZE, "It does not support ignoreSize and numberOfTicks parameters in tick-by-tick requests.") == false {
            return;
        }

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::ReqTickByTickData as i32);
        params_list.add_int(req_id);
        params_list.add_int(contract.con_id);
        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());
        params_list.add_string(contract.multiplier.as_str());
        params_list.add_string(contract.exchange.as_str());
        params_list.add_string(contract.primary_exchange.as_str());
        params_list.add_string(contract.currency.as_str());
        params_list.add_string(contract.local_symbol.as_str());
        params_list.add_string(contract.trading_class.as_str());
        params_list.add_string(tick_type);

        if self.client_socket.server_version >= min_server_version::TICK_BY_TICK_IGNORE_SIZE
        {
            params_list.add_int(number_of_ticks);
            params_list.add_bool(ignore_size);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQTICKBYTICKDATA, err.to_string().as_str());
        });
    }

    pub fn req_pnl(&mut self, req_id: i32, account: &str, model_code: &str) {
        if self.check_connection() == false {
            return;
        }

        if self.check_server_version(req_id, min_server_version::PNL, "It does not support PNL request") == false {
            return;
        }


        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::ReqPnL as i32);
        params_list.add_int(req_id);
        params_list.add_string(account);
        params_list.add_string(model_code);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQTICKBYTICKDATA, err.to_string().as_str());
        });
    }

    pub fn cancel_pnl(&mut self, req_id: i32) {
        if self.check_connection() == false {
            return;
        }

        if self.check_server_version(req_id, min_server_version::PNL, "It does not support PNL request") == false {
            return;
        }

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::CancelPnL as i32);
        params_list.add_int(req_id);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_REQTICKBYTICKDATA, err.to_string().as_str());
        });
    }

    pub fn cancel_scanner_subscription(&mut self, req_id: i32) {
        if self.check_connection() == false {
            return;
        }

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::CancelScannerSubscription as i32);
        params_list.add_int(1);
        params_list.add_int(req_id);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_CANSCANNER, err.to_string().as_str());
        });
    }

    pub fn cancel_tick_subscription(&mut self, req_id: i32) {
        if self.check_connection() == false {
            return;
        }

        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::CancelTickByTickData as i32);
        params_list.add_int(req_id);

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(req_id, client_errors::FAIL_SEND_CANCELTICKBYTICKDATA, err.to_string().as_str());
        });
    }

    pub fn place_order(&mut self, order_id: i32, contract: Contract, mut order: Order) {
        if self.check_connection() == false {
            return;
        }

        let is_bag_order = IbClient::strings_are_equal(helper_constants::BAG_SEC_TYPE, &contract.sec_type);
        if self.verify_order(&order, order_id, is_bag_order) == false {
            return;
        }

        if self.verify_order_contract(&contract, order_id) == false {
            return
        }

        let message_version = if self.client_socket.server_version < min_server_version::NOT_HELD { 27 } else { 45 };
        let mut params_list = ApiParameters::new();
        let length_pos = params_list.prepare_buffer(self.client_socket.use_v1000_plus);

        params_list.add_int(OutgoingMessages::PlaceOrder as i32);

        if self.client_socket.server_version < min_server_version::ORDER_CONTAINER {
            params_list.add_int(message_version);
        }

        params_list.add_int(order_id);

        if self.client_socket.server_version >= min_server_version::PLACE_ORDER_CONID {
            params_list.add_int(contract.con_id);
        }

        params_list.add_string(contract.symbol.as_str());
        params_list.add_string(contract.sec_type.as_str());
        params_list.add_string(contract.last_trade_date_or_contract_month.as_str());
        params_list.add_double(contract.strike);
        params_list.add_string(contract.right.as_str());
        if self.client_socket.server_version >= 15 {
            params_list.add_string(contract.multiplier.as_str());
        }
        params_list.add_string(contract.exchange.as_str());
        if self.client_socket.server_version >= 14 {
            params_list.add_string(contract.primary_exchange.as_str());
        }
        params_list.add_string(contract.currency.as_str());
        if self.client_socket.server_version >= 2 {
            params_list.add_string(contract.local_symbol.as_str());
        }

        if self.client_socket.server_version >= min_server_version::TRADING_CLASS
        {
            params_list.add_string(contract.trading_class.as_str());
        }
        if self.client_socket.server_version >= min_server_version::SEC_ID_TYPE
        {
            params_list.add_string(contract.sec_id_type.as_str());
            params_list.add_string(contract.sec_id.as_str());
        }

        // params_list.add_string main order fields
        params_list.add_string(order.action.as_str());

        if self.client_socket.server_version >= min_server_version::FRACTIONAL_POSITIONS {
            params_list.add_double(order.total_quantity);
        }
        else {
            params_list.add_int(order.total_quantity as i32);
        }

        params_list.add_string(order.order_type.as_str());
        if self.client_socket.server_version < min_server_version::ORDER_COMBO_LEGS_PRICE
        {
            let val = if order.lmt_price == f64::MAX { 0.0 } else { order.lmt_price };
            params_list.add_double(val);
        }
        else
        {
            params_list.add_double_max(order.lmt_price);
        }
        if self.client_socket.server_version < min_server_version::TRAILING_PERCENT
        {
            let val = if order.aux_price == f64::MAX { 0.0 } else { order.aux_price };
            params_list.add_double(val);
        }
        else
        {
            params_list.add_double_max(order.aux_price);
        }

        // params_list.add_string extended order fields
        params_list.add_string(order.tif.as_str());
        params_list.add_string(order.oca_group.as_str());
        params_list.add_string(order.account.as_str());
        params_list.add_string(order.open_close.as_str());
        params_list.add_int(order.origin as i32);
        params_list.add_string(order.order_ref.as_str());
        params_list.add_bool(order.transmit);
        if self.client_socket.server_version >= 4
        {
            params_list.add_int(order.parent_id);
        }

        if self.client_socket.server_version >= 5
        {
            params_list.add_bool(order.block_order);
            params_list.add_bool(order.sweep_to_fill);
            params_list.add_int(order.display_size);
            params_list.add_int(order.trigger_method);
            if self.client_socket.server_version < 38
            {
                // will never happen
                params_list.add_bool(/* order.ignoreRth */ false);
            }
            else
            {
                params_list.add_bool(order.outside_rth);
            }
        }

        if self.client_socket.server_version >= 7
        {
            params_list.add_bool(order.hidden);
        }

        // params_list.add_string combo legs for BAG requests
        let is_bag = IbClient::strings_are_equal(helper_constants::BAG_SEC_TYPE, &contract.sec_type);
        if self.client_socket.server_version >= 8 && is_bag
        {
            if contract.combo_legs.len() == 0
            {
                params_list.add_int(0);
            }
            else
            {
                params_list.add_int(contract.combo_legs.len() as i32);
                for combo_leg in contract.combo_legs.iter() {
                    params_list.add_int(combo_leg.con_id);
                    params_list.add_int(combo_leg.ratio);
                    params_list.add_string(combo_leg.action.as_str());
                    params_list.add_string(combo_leg.exchange.as_str());
                    params_list.add_int((combo_leg.open_close).clone() as i32);

                    if self.client_socket.server_version >= min_server_version::SSHORT_COMBO_LEGS
                    {
                        params_list.add_int(combo_leg.short_sale_slot);
                        params_list.add_string(combo_leg.designated_location.as_str());
                    }
                    if self.client_socket.server_version >= min_server_version::SSHORTX_OLD
                    {
                        params_list.add_int(combo_leg.exempt_code);
                    }
                }
            }
        }

        // add order combo legs for BAG requests
        if self.client_socket.server_version >= min_server_version::ORDER_COMBO_LEGS_PRICE && is_bag
        {
            if order.order_combo_legs.len() == 0
            {
                params_list.add_int(0);
            }
            else
            {
                params_list.add_int(order.order_combo_legs.len() as i32);

                for order_combo_leg in order.order_combo_legs.iter() {
                    params_list.add_double(order_combo_leg.price);
                }
            }
        }

        if self.client_socket.server_version >= min_server_version::SMART_COMBO_ROUTING_PARAMS && is_bag
        {
            params_list.add_int(order.smart_combo_routing_params.len() as i32);
            if order.smart_combo_routing_params.len() > 0
            {
                for smart_combo_routing_param in order.smart_combo_routing_params.iter() {
                    params_list.add_string(smart_combo_routing_param.tag.as_str());
                    params_list.add_string(smart_combo_routing_param.value.as_str());
                }
            }
        }

        if self.client_socket.server_version >= 9
        {
            // params_list.add_string deprecated sharesAllocation field
            params_list.add_string("");
        }

        if self.client_socket.server_version >= 10
        {
            params_list.add_double(order.discretionary_amt);
        }

        if self.client_socket.server_version >= 11
        {
            params_list.add_string(order.good_after_time.as_str());
        }

        if self.client_socket.server_version >= 12
        {
            params_list.add_string(order.good_till_date.as_str());
        }

        if self.client_socket.server_version >= 13
        {
            params_list.add_string(order.fa_group.as_str());
            params_list.add_string(order.fa_method.as_str());
            params_list.add_string(order.fa_percentage.as_str());
            params_list.add_string(order.fa_profile.as_str());
        }

        if self.client_socket.server_version >= min_server_version::MODELS_SUPPORT
        {
            params_list.add_string(order.model_code.as_str());
        }

        if self.client_socket.server_version >= 18
        { // institutional short sale slot fields.
            params_list.add_int(order.short_sale_slot);      // 0 only for retail, 1 or 2 only for institution.
            params_list.add_string(order.designated_location.as_str()); // only populate when order.shortSaleSlot = 2.
        }
        if self.client_socket.server_version >= min_server_version::SSHORTX_OLD
        {
            params_list.add_int(order.exempt_code);
        }
        if self.client_socket.server_version >= 19
        {
            params_list.add_int(order.oca_type);
            if self.client_socket.server_version < 38
            {
                // will never happen
                params_list.add_bool( /* order.rthOnly */ false);
            }
            params_list.add_string(order.rule80a.as_str());
            params_list.add_string(order.settling_firm.as_str());
            params_list.add_bool(order.all_or_none);
            params_list.add_int_max(order.min_qty);
            params_list.add_double_max(order.percent_offset);
            params_list.add_bool(order.e_trade_only);
            params_list.add_bool(order.firm_quote_only);
            params_list.add_double_max(order.nbbo_price_cap);
            params_list.add_int_max(order.auction_strategy);
            params_list.add_double_max(order.starting_price);
            params_list.add_double_max(order.stock_ref_price);
            params_list.add_double_max(order.delta);
            // Volatility orders had specific watermark price attribs in server version 26
            let lower = if self.client_socket.server_version == 26 && order.order_type == "VOL".to_string() { f64::MAX } else { order.stock_range_lower };
            let upper = if self.client_socket.server_version == 26 && order.order_type == "VOL".to_string() { f64::MAX } else { order.stock_range_upper };
            params_list.add_double_max(lower);
            params_list.add_double_max(upper);
        }

        if self.client_socket.server_version >= 22
        {
            params_list.add_bool(order.override_percentage_constraints);
        }

        if self.client_socket.server_version >= 26
        { // Volatility orders
            params_list.add_double_max(order.volatility);
            params_list.add_int_max(order.volatility_type);
            if self.client_socket.server_version < 28
            {
                let is_delta_neutral_type_mkt = IbClient::strings_are_equal("MKT", &order.delta_neutral_order_type);
                params_list.add_bool(is_delta_neutral_type_mkt);
            }
            else
            {
                params_list.add_string(order.delta_neutral_order_type.as_str());
                params_list.add_double_max(order.delta_neutral_aux_price);

                if self.client_socket.server_version >= min_server_version::DELTA_NEUTRAL_CONID && order.delta_neutral_order_type.is_empty() == false
                {
                    params_list.add_int(order.delta_neutral_con_id);
                    params_list.add_string(order.delta_neutral_settling_firm.as_str());
                    params_list.add_string(order.delta_neutral_clearing_account.as_str());
                    params_list.add_string(order.delta_neutral_clearing_intent.as_str());
                }

                if self.client_socket.server_version >= min_server_version::DELTA_NEUTRAL_OPEN_CLOSE && order.delta_neutral_order_type.is_empty() == false
                {
                    params_list.add_string(order.delta_neutral_open_close.as_str());
                    params_list.add_bool(order.delta_neutral_short_sale);
                    params_list.add_int(order.delta_neutral_short_sale_slot);
                    params_list.add_string(order.delta_neutral_designated_location.as_str());
                }
            }
            params_list.add_int(order.continuous_update);
            if self.client_socket.server_version == 26
            {
                // Volatility orders had specific watermark price attribs in server version 26
                let lower = if order.order_type == "VOL".to_string() { order.stock_range_lower } else { f64::MAX };
                let upper = if order.order_type == "VOL".to_string() { order.stock_range_upper } else { f64::MAX };
                params_list.add_double_max(lower);
                params_list.add_double_max(upper);
            }
            params_list.add_int_max(order.reference_price_type);
        }

        if self.client_socket.server_version >= 30
        { // TRAIL_STOP_LIMIT stop price
            params_list.add_double_max(order.trail_stop_price);
        }

        if self.client_socket.server_version >= min_server_version::TRAILING_PERCENT
        {
            params_list.add_double_max(order.trailing_percent);
        }

        if self.client_socket.server_version >= min_server_version::SCALE_ORDERS
        {
            if self.client_socket.server_version >= min_server_version::SCALE_ORDERS2
            {
                params_list.add_int_max(order.scale_init_level_size);
                params_list.add_int_max(order.scale_subs_level_size);
            }
            else
            {
                params_list.add_string("");
                params_list.add_int_max(order.scale_init_level_size);

            }
            params_list.add_double_max(order.scale_price_increment);
        }

        if self.client_socket.server_version >= min_server_version::SCALE_ORDERS3 && order.scale_price_increment > 0.0 && order.scale_price_increment != f64::MAX
        {
            params_list.add_double_max(order.scale_price_adjust_value);
            params_list.add_int_max(order.scale_price_adjust_interval);
            params_list.add_double_max(order.scale_profit_offset);
            params_list.add_bool(order.scale_auto_reset);
            params_list.add_int_max(order.scale_init_position);
            params_list.add_int_max(order.scale_init_fill_qty);
            params_list.add_bool(order.scale_random_percent);
        }

        if self.client_socket.server_version >= min_server_version::SCALE_TABLE
        {
            params_list.add_string(order.scale_table.as_str());
            params_list.add_string(order.active_start_time.as_str());
            params_list.add_string(order.active_stop_time.as_str());
        }

        if self.client_socket.server_version >= min_server_version::HEDGE_ORDERS
        {
            params_list.add_string(order.hedge_type.as_str());
            if order.hedge_type.is_empty() == false
            {
                params_list.add_string(order.hedge_param.as_str());
            }
        }

        if self.client_socket.server_version >= min_server_version::OPT_OUT_SMART_ROUTING
        {
            params_list.add_bool(order.opt_out_smart_routing);
        }

        if self.client_socket.server_version >= min_server_version::PTA_ORDERS
        {
            params_list.add_string(order.clearing_account.as_str());
            params_list.add_string(order.clearing_intent.as_str());
        }

        if self.client_socket.server_version >= min_server_version::NOT_HELD
        {
            params_list.add_bool(order.not_held);
        }

        if self.client_socket.server_version >= min_server_version::DELTA_NEUTRAL
        {
            match contract.delta_neutral_contract {
                Some(res) => {
                    params_list.add_bool(true);
                    params_list.add_int(res.con_id);
                    params_list.add_double(res.delta);
                    params_list.add_double(res.price);

                },
                None => {
                    params_list.add_bool(false);

                }
            }
        }

        if self.client_socket.server_version >= min_server_version::ALGO_ORDERS
        {
            params_list.add_string(order.algo_strategy.as_str());
            if order.algo_strategy.is_empty() == false
            {
                params_list.add_int(order.algo_params.len() as i32);
                for algo_param in order.algo_params.iter() {
                    params_list.add_string(algo_param.tag.as_str());
                    params_list.add_string(algo_param.value.as_str());
                }
            }
        }

        if self.client_socket.server_version >= min_server_version::ALGO_ID
        {
            params_list.add_string(order.algo_id.as_str());
        }

        if self.client_socket.server_version >= min_server_version::WHAT_IF_ORDERS
        {
            params_list.add_bool(order.what_if);
        }

        if self.client_socket.server_version >= min_server_version::LINKING
        {
            let mut tag_values = "".to_string();

            for item in order.order_misc_options {
                tag_values += format!("{}={};", item.tag, item.value).as_ref();
            }

            params_list.add_string(tag_values.as_str());
        }

        if self.client_socket.server_version >= min_server_version::ORDER_SOLICITED
        {
            params_list.add_bool(order.solicited);
        }

        if self.client_socket.server_version >= min_server_version::RANDOMIZE_SIZE_AND_PRICE
        {
            params_list.add_bool(order.randomize_size);
            params_list.add_bool(order.randomize_price);
        }

        if self.client_socket.server_version >= min_server_version::PEGGED_TO_BENCHMARK
        {
            if order.order_type == "PEG BENCH".to_string()
            {
                params_list.add_int(order.reference_contract_id);
                params_list.add_bool(order.is_pegged_change_amount_decrease);
                params_list.add_double(order.pegged_change_amount);
                params_list.add_double(order.reference_change_amount);
                params_list.add_string(order.reference_exchange.as_str());
            }

            params_list.add_int(order.conditions.len() as i32);

            if order.conditions.len() > 0
            {
                for condition in order.conditions.iter_mut() {
                    params_list.add_int(condition.get_type());
                    condition.serialize(&mut params_list);
                }

                params_list.add_bool(order.conditions_ignore_rth);
                params_list.add_bool(order.conditions_cancel_order);
            }

            params_list.add_string(order.adjusted_order_type.as_str());
            params_list.add_double(order.trigger_price);
            params_list.add_double(order.lmt_price_offset);
            params_list.add_double(order.adjusted_stop_price);
            params_list.add_double(order.adjusted_stop_limit_price);
            params_list.add_double(order.adjusted_trailing_amount);
            params_list.add_int(order.adjustable_trailing_unit);
        }

        if self.client_socket.server_version >= min_server_version::EXT_OPERATOR
        {
            params_list.add_string(order.ext_operator.as_str());
        }

        if self.client_socket.server_version >= min_server_version::SOFT_DOLLAR_TIER
        {
            params_list.add_string(order.soft_dollar_tier.name.as_str());
            params_list.add_string(order.soft_dollar_tier.val.as_str());
        }

        if self.client_socket.server_version >= min_server_version::CASH_QTY
        {
            params_list.add_double(order.cash_qty);
        }

        if self.client_socket.server_version >= min_server_version::DECISION_MAKER
        {
            params_list.add_string(order.mifid2decision_maker.as_str());
            params_list.add_string(order.mifid2decision_algo.as_str());
        }

        if self.client_socket.server_version >= min_server_version::MIFID_EXECUTION
        {
            params_list.add_string(order.mifid2execution_trader.as_str());
            params_list.add_string(order.mifid2execution_algo.as_str());
        }

        if self.client_socket.server_version >= min_server_version::AUTO_PRICE_FOR_HEDGE
        {
            params_list.add_bool(order.dont_use_auto_price_for_hedge);
        }

        if self.client_socket.server_version >= min_server_version::ORDER_CONTAINER
        {
            params_list.add_bool(order.is_oms_container);
        }

        if self.client_socket.server_version >= min_server_version::D_PEG_ORDERS
        {
            params_list.add_bool(order.discretionary_up_to_limit_price);
        }

        if self.client_socket.server_version >= min_server_version::PRICE_MGMT_ALGO
        {
            params_list.add_bool(order.use_price_mgmt_algo);
        }

        self.client_socket.close_and_send(&mut params_list, length_pos).unwrap_or_else(|err| {
            self.report_error(order_id, client_errors::FAIL_SEND_ORDER, err.to_string().as_str());
        })
    }

    pub fn stop(&self) {
        let msg = IncomingMessagesEnum::Stop;
        self.event_sender.send(msg).unwrap_or_else(|err| {
            self.report_error(700, client_errors::FAIL_SEND_ORDER, err.to_string().as_str());
        });
    }

    fn strings_are_equal(a: &str, b: &str) -> bool {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();
        a_lower == b_lower
    }

    fn verify_order(&self, order: &Order, id: i32, is_bag_order: bool) -> bool {
        if self.client_socket.server_version < min_server_version::SCALE_ORDERS {
            if order.scale_init_level_size != i32::MAX || order.scale_price_increment != f64::MAX {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support Scale orders");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::WHAT_IF_ORDERS {
            if order.what_if {
                self.report_error(id, client_errors::UPDATE_TWS, "it does not support what-if orders");
            }
        }

        if self.client_socket.server_version < min_server_version::SCALE_ORDERS {
            if order.scale_subs_level_size != i32::MAX {
                self.report_error(id, client_errors::UPDATE_TWS, "it does not support subsequent Level Size for Scale Orders.");
            }
        }

        if self.client_socket.server_version < min_server_version::ALGO_ORDERS
        {
            if order.algo_strategy.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support algo orders.");
                return false
            }
        }

        if self.client_socket.server_version < min_server_version::NOT_HELD
        {
            if order.not_held
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support notHeld parameter.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::SSHORTX
        {
            if order.exempt_code != -1
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support exemptCode parameter.");
                return false;
            }
        }



        if self.client_socket.server_version < min_server_version::HEDGE_ORDERS
        {
            if order.hedge_type.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support hedge orders.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::OPT_OUT_SMART_ROUTING
        {
            if order.opt_out_smart_routing
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support optOutSmartRouting parameter.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::DELTA_NEUTRAL_CONID
        {
            if order.delta_neutral_con_id > 0 || order.delta_neutral_settling_firm.is_empty() == false || order.delta_neutral_clearing_account.is_empty() == false || order.delta_neutral_clearing_intent.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support deltaNeutral parameters: ConId, SettlingFirm, ClearingAccount, ClearingIntent");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::DELTA_NEUTRAL_OPEN_CLOSE
        {
            if order.delta_neutral_open_close.is_empty() == false || order.delta_neutral_short_sale || order.delta_neutral_short_sale_slot > 0 || order.delta_neutral_designated_location.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support deltaNeutral parameters: OpenClose, ShortSale, ShortSaleSlot, DesignatedLocation");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::SCALE_ORDERS3
        {
            if order.scale_price_increment > 0.0 && order.scale_price_increment != f64::MAX
            {
                if order.scale_price_adjust_value != f64::MAX ||
                    order.scale_price_adjust_interval != i32::MAX ||
                    order.scale_profit_offset != f64::MAX ||
                    order.scale_auto_reset ||
                    order.scale_init_position != i32::MAX ||
                    order.scale_init_fill_qty != i32::MAX ||
                    order.scale_random_percent
                {
                    self.report_error(id, client_errors::UPDATE_TWS, "It does not support Scale order parameters: PriceAdjustValue, PriceAdjustInterval, ProfitOffset, AutoReset, InitPosition, InitFillQty and RandomPercent");
                    return false;
                }
            }
        }

        if self.client_socket.server_version < min_server_version::ORDER_COMBO_LEGS_PRICE && is_bag_order
        {
            if order.order_combo_legs.len() > 0
            {
                for order_combo_leg in order.order_combo_legs.iter() {
                    if order_combo_leg.price != f64::MAX {
                        self.report_error(id, client_errors::UPDATE_TWS, "It does not support per-leg prices for order combo legs.");
                        return false;
                    }
                }
            }
        }

        if self.client_socket.server_version < min_server_version::TRAILING_PERCENT
        {
            if order.trailing_percent != f64::MAX
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support trailing percent parameter.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::ALGO_ID && order.algo_id.is_empty() == false
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support algoId parameter");

            return false;
        }

        if self.client_socket.server_version < min_server_version::SCALE_TABLE
        {
            if order.scale_table.is_empty() == false || order.active_start_time.is_empty() == false || order.active_stop_time.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support scaleTable, activeStartTime nor activeStopTime parameters.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::EXT_OPERATOR && order.ext_operator.is_empty() == false
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support extOperator parameter");
            return false;
        }

        if self.client_socket.server_version < min_server_version::CASH_QTY && order.cash_qty != f64::MAX
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support cashQty parameter");
            return false;
        }

        if self.client_socket.server_version < min_server_version::DECISION_MAKER && (order.mifid2decision_maker.is_empty() || order.mifid2decision_algo.is_empty())
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support MIFID II decision maker parameters");
            return false;
        }

        if self.client_socket.server_version < min_server_version::DECISION_MAKER && (order.mifid2execution_trader.is_empty() == false || order.mifid2execution_algo.is_empty() == false)
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support MIFID II execution parameters");
            return false;
        }

        if self.client_socket.server_version < min_server_version::AUTO_PRICE_FOR_HEDGE && order.dont_use_auto_price_for_hedge
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support don't use auto price for hedge parameter");
            return false;
        }

        if self.client_socket.server_version < min_server_version::ORDER_CONTAINER && order.is_oms_container
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support oms container parameter.");
            return false;
        }

        if self.client_socket.server_version < min_server_version::D_PEG_ORDERS && order.discretionary_up_to_limit_price
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support D-Peg orders.");
            return false;
        }

        if self.client_socket.server_version < min_server_version::PRICE_MGMT_ALGO
        {
            self.report_error(id, client_errors::UPDATE_TWS, "It does not support Use Price Management Algo requests.");
            return false;
        }

        true
    }

    fn verify_order_contract(&self, contract: &Contract, id: i32) -> bool {
        if self.client_socket.server_version < min_server_version::SSHORT_COMBO_LEGS
        {
            if contract.combo_legs.len() > 0
            {
                for combo_leg in contract.combo_legs.iter() {
                    if combo_leg.short_sale_slot != 0 || combo_leg.designated_location.is_empty() == false {
                        self.report_error(id, client_errors::UPDATE_TWS, "It does not support SSHORT flag for combo legs.");
                        return false;
                    }
                }
            }
        }

        if self.client_socket.server_version < min_server_version::DELTA_NEUTRAL
        {
            if contract.delta_neutral_contract.is_some()
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support delta-neutral orders.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::PLACE_ORDER_CONID
        {
            if contract.con_id > 0
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support conId parameter.");
                return false;
            }
        }

        if self.client_socket.server_version < min_server_version::SEC_ID_TYPE
        {
            if contract.sec_id_type.is_empty() == false || contract.sec_id.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support secIdType and secId parameters.");
                return false;
            }
        }
        if self.client_socket.server_version < min_server_version::SSHORTX
        {
            if contract.combo_legs.len() > 0
            {
                for combo_leg in contract.combo_legs.iter() {
                    if combo_leg.exempt_code != -1 {
                        self.report_error(id, client_errors::UPDATE_TWS, "It does not support exemptCode parameter.");
                        return false;
                    }
                }
            }
        }
        if self.client_socket.server_version < min_server_version::TRADING_CLASS
        {
            if contract.trading_class.is_empty() == false
            {
                self.report_error(id, client_errors::UPDATE_TWS, "It does not support tradingClass parameters in placeOrder.");
                return false;
            }
        }
        true
    }

    fn report_error(&self, id: i32, error: (i32, &str), tail: &str) {
        let error_message = format!("{} {}", error.1, tail);
        let enum_error = IncomingMessagesEnum::Error(id, error.0, error_message);
        self.event_sender.send(enum_error).unwrap();
    }

    fn check_server_version(&self, req_id: i32, required_version: i32, update_tail: &str) -> bool {
        if self.client_socket.server_version < required_version {
            self.report_update_tws(req_id, update_tail);
        }
        return true;
    }

    fn report_update_tws(&self, req_id: i32, tail: &str) {
        self.report_error(req_id, client_errors::UPDATE_TWS, tail);
    }

    fn check_connection(&mut self) -> bool {
        if self.is_connected == false
        {
            self.report_error(client_errors::NOT_CONNECTED.0, client_errors::NOT_CONNECTED, "");
            return false;
        }
        return true;
    }
}
