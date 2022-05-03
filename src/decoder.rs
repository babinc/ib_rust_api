use std::io::{Cursor, Read, ErrorKind};
use crossbeam_channel::{Sender};
use crate::models::contract_details::ContractDetails;
use crate::models::scan_data_item::ScanDataItem;
use crate::constants::{incoming_message_constants, min_server_version};
use crate::enums::incoming_message_enum::IncomingMessagesEnum;
use crate::models::account_summary::AccountSummary;
use crate::models::contract::Contract;
use crate::models::order::Order;
use crate::models::order_state::OrderState;
use crate::order_decoder::OrderDecoder;
use crate::models::order_data_item::OrderDataItem;
use crate::models::tag_value::TagValue;
use crate::models::order_status_message::OrderStatusMessage;
use crate::models::bar::Bar;
use std::str::FromStr;
use std::slice::Iter;
use std::error::Error;
use crate::constants::helper_constants::{UNSET_INTEGER, UNSET_DOUBLE};
use crate::models::tick_last::TickLast;

pub struct Decoder {
    fields: Vec<String>
}

pub fn decode_i32(iter: &mut Iter<String>) -> Result<i32, Box<dyn Error>> {
    let next = iter.next();

    let val: i32 = next.unwrap().parse().unwrap_or(0);
    Ok(val)
}

pub fn decode_i32_show_unset(iter: &mut Iter<String>) -> Result<i32, Box<dyn Error>> {
    let next = iter.next();
    let retval: i32 = next.unwrap().parse().unwrap_or(0);
    Ok(if retval == 0 { UNSET_INTEGER } else { retval })
}

pub fn decode_i64(iter: &mut Iter<String>) -> Result<i64, Box<dyn Error>> {
    let next = iter.next();
    let val: i64 = next.unwrap().parse().unwrap_or(0);
    Ok(val)
}

pub fn decode_f64(iter: &mut Iter<String>) -> Result<f64, Box<dyn Error>> {
    let next = iter.next();
    let val = next.unwrap().parse().unwrap_or(0.0);
    Ok(val)
}

pub fn decode_f64_show_unset(iter: &mut Iter<String>) -> Result<f64, Box<dyn Error>> {
    let next = iter.next();
    let rtn: f64 = next.unwrap().parse().unwrap_or(0.0);
    Ok(if rtn == 0.0 { UNSET_DOUBLE } else { rtn })
}

pub fn decode_string(iter: &mut Iter<String>) -> Result<String, Box<dyn Error>> {
    let next = iter.next();
    let val = next.unwrap().parse().unwrap_or("".to_string());
    Ok(val)
}

pub fn decode_bool(iter: &mut Iter<String>) -> Result<bool, Box<dyn Error>> {
    let next = iter.next();
    //info!("{:?}", next);
    let retval: i32 = next.unwrap_or(&"0".to_string()).parse().unwrap_or(0);
    Ok(retval != 0)
}

impl Decoder {
    pub fn new(fields: &[String]) -> Decoder {
        Decoder {
            fields: fields.to_vec()
        }
    }

    pub fn process_connect_ack(mut msg: Cursor<Vec<u8>>, server_version: &mut i32, server_time: &mut String, is_connected: &mut bool) {
        *server_version = Self::read_int(&mut msg);

        if *server_version == -1 {
            unimplemented!()
        }

        if *server_version > 20 {
            *server_time = Self::read_string(&mut msg).unwrap_or_default();
        }

        *is_connected = true;
    }

    pub fn process_incoming_message(&mut self, server_version: i32, sender_messages_enum: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        if self.fields.is_empty() {
            return Err(
                Box::new(std::io::Error::new(ErrorKind::Other, "incoming message is empty"))
            );
        }

        let message = i32::from_str(self.fields.get(0).unwrap().as_str()).unwrap();

        if message == incoming_message_constants::NOT_VALID || message == 0 {
            return Err(
                Box::new(std::io::Error::new(ErrorKind::Other, "incoming message not valid"))
            );
        }

        match message {
            incoming_message_constants::ERROR => {
                let error = self.error_event()?;
                let enum_error = IncomingMessagesEnum::Error(error.0, error.1, error.2);
                sender_messages_enum.send(enum_error)?;
            },
            incoming_message_constants::NEXT_VALID_ID => {
                let next_id = self.next_valid_id()?;
                let enum_next_id = IncomingMessagesEnum::NextValidId(next_id);
                sender_messages_enum.send(enum_next_id)?;
            },
            incoming_message_constants::MANAGED_ACCOUNTS => {
                let result = self.managed_accounts_event()?;
                let enum_accounts_list = IncomingMessagesEnum::ManagedAccounts(result);
                sender_messages_enum.send(enum_accounts_list)?;
            },
            incoming_message_constants::SCANNER_DATA => {
                let items = self.scanner_data_event()?;
                let msg_enum = IncomingMessagesEnum::ScannerData(items);
                sender_messages_enum.send(msg_enum)?;
            },
            incoming_message_constants::ACCOUNT_SUMMARY => {
                let (request_id, account, tag, value, currency) = self.account_summary_event()?;
                let account_summary = AccountSummary {
                    request_id,
                    account,
                    tag,
                    value,
                    currency
                };
                sender_messages_enum.send(IncomingMessagesEnum::AccountSummary(account_summary))?;
            },
            incoming_message_constants::ACCOUNT_SUMMARY_END => {
                let request_id = self.account_summary_event_end()?;
                sender_messages_enum.send(IncomingMessagesEnum::AccountSummaryEnd(request_id))?;
            },
            incoming_message_constants::OPEN_ORDER => {
                let order_data = self.open_order_event(server_version)?;
                sender_messages_enum.send(IncomingMessagesEnum::OpenOrder(order_data))?;
            },
            incoming_message_constants::OPEN_ORDER_END => {
                sender_messages_enum.send(IncomingMessagesEnum::OpenOrderEnd)?;
            },
            incoming_message_constants::CONTRACT_DATA => {
                let (req_id, data) = self.contract_data_event(server_version)?;
                sender_messages_enum.send(IncomingMessagesEnum::ContractData(req_id, data))?;
            },
            incoming_message_constants::CONTRACT_DATA_END => {
                let mut fields_itr = self.fields.iter();
                let _msg_version = decode_i32(&mut fields_itr)?;
                let request_id = decode_i32(&mut fields_itr)?;
                sender_messages_enum.send(IncomingMessagesEnum::ContractDataEnd(request_id))?;
            },
            incoming_message_constants::ORDER_STATUS => {
                let order_status_message = self.order_status_event(server_version)?;
                sender_messages_enum.send(IncomingMessagesEnum::OrderStatus(order_status_message))?;
            },
            incoming_message_constants::HISTORICAL_DATA => {
                 self.historical_data_event(server_version, &sender_messages_enum)?;
            },
            incoming_message_constants::REAL_TIME_BARS => {
                self.read_time_bars_event(&sender_messages_enum)?;
            },
            incoming_message_constants::TICK_BY_TICK => {
                self.tick_by_tick_event(&sender_messages_enum)?;
            },
            incoming_message_constants::TICK_GENERIC => {
                self.generic_tick(&sender_messages_enum)?;
            },
            incoming_message_constants::PN_L => {
                self.profit_and_losses(server_version, &sender_messages_enum)?;
            },
            _ => {}
        }

        Ok(())
    }

    pub fn read_int(buf: &mut Cursor<Vec<u8>>) -> i32 {
        return match Self::read_string(buf) {
            Ok(res) => {
                if res.is_empty() {
                    0
                }
                else {
                    match res.parse::<i32>() {
                        Ok(res) => res,
                        Err(err) => {
                            eprintln!("Error reading int {}", err);
                            0
                        }
                    }
                }
            },
            Err(err) => {
                eprintln!("{}", err.to_string());
                0
            }
        }
    }

    pub fn read_string(buf: &mut Cursor<Vec<u8>>) -> Result<String, std::io::Error> {
        let mut b: [u8; 1] = [0_u8; 1];
        buf.read(&mut b)?;

        if b[0] == 0  {
            return Err(std::io::Error::new(ErrorKind::Other, format!("read_string buffer has zero length")));
        }

        let mut str_bytes: Vec<u8> = Vec::new();
        str_bytes.push(b[0]);

        loop {
            b[0] = 0;
            buf.read(&mut b)?;
            if b[0] == 0 {
                break;
            }
            str_bytes.push(b[0]);
        }

        let result_str = std::str::from_utf8(str_bytes.as_slice()).unwrap();

        Ok(result_str.to_string())
    }

    fn error_event(&mut self) -> Result<(i32, i32, String), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let id = decode_i32(&mut fields_itr)?;
        let error_code = decode_i32(&mut fields_itr)?;
        let error_message = decode_string(&mut fields_itr)?;

        Ok((id, error_code, error_message))
    }

    fn next_valid_id(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let order_id = decode_i32(&mut fields_itr)?;

        Ok(order_id)
    }

    fn scanner_data_event(&mut self) -> Result<Vec<ScanDataItem>, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let req_id = decode_i32(&mut fields_itr)?;

        let number_of_elements = decode_i32(&mut fields_itr)?;

        let mut rtn: Vec<ScanDataItem> = vec!();

        for _ in 0..number_of_elements {
            let mut contract_details = ContractDetails::new();

            let rank = decode_i32(&mut fields_itr)?;
            contract_details.contract.con_id = decode_i32(&mut fields_itr)?;
            contract_details.contract.symbol = decode_string(&mut fields_itr)?;
            contract_details.contract.sec_type = decode_string(&mut fields_itr)?;
            contract_details.contract.last_trade_date_or_contract_month = decode_string(&mut fields_itr)?;
            contract_details.contract.strike = decode_f64(&mut fields_itr)?;
            contract_details.contract.right = decode_string(&mut fields_itr)?;
            contract_details.contract.exchange = decode_string(&mut fields_itr)?;
            contract_details.contract.currency = decode_string(&mut fields_itr)?;
            contract_details.contract.local_symbol = decode_string(&mut fields_itr)?;
            contract_details.market_name = decode_string(&mut fields_itr)?;
            contract_details.contract.trading_class = decode_string(&mut fields_itr)?;
            let distance = decode_string(&mut fields_itr)?;
            let benchmark = decode_string(&mut fields_itr)?;
            let projection = decode_string(&mut fields_itr)?;
            let legs_str = decode_string(&mut fields_itr)?;

            let item = ScanDataItem {
                request_id: req_id,
                rank,
                contract_details,
                distance,
                benchmark,
                projection,
                legs_str
            };

            rtn.push(item);
        }

        Ok(rtn )
    }

    fn managed_accounts_event(&mut self) -> Result<String, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let accounts_list = decode_string(&mut fields_itr)?;

        Ok(accounts_list)
    }

    fn account_summary_event(&mut self) -> Result<(i32, String, String, String, String), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next().unwrap();
        //throw away version
        fields_itr.next().unwrap();

        let request_id = decode_i32(&mut fields_itr)?;
        let account_id = decode_string(&mut fields_itr)?;
        let tag = decode_string(&mut fields_itr)?;
        let value = decode_string(&mut fields_itr)?;
        let currency = decode_string(&mut fields_itr)?;

        Ok((request_id, account_id, tag, value, currency))
    }

    fn account_summary_event_end(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let request_id = decode_i32(&mut fields_itr)?;
        Ok(request_id)
    }

    fn open_order_event(&mut self, server_version: i32) -> Result<OrderDataItem, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        let msg_version = if server_version < min_server_version::ORDER_CONTAINER {
            decode_i32(&mut fields_itr)?
        }
        else {
            server_version
        };

        if server_version < min_server_version::ORDER_CONTAINER {
            eprintln!("Server version does not support order Container. User version > {}", min_server_version::ORDER_CONTAINER);
        }

        let mut order = Order::new();
        let mut contract = Contract::new();
        let mut order_state = OrderState::new();
        let mut order_decoder = OrderDecoder::new(&mut contract, &mut order, &mut order_state, msg_version, server_version, fields_itr);

        order_decoder.read_order_id()?;
        order_decoder.read_contract_fields()?;

        order_decoder.read_action()?;
        order_decoder.read_total_quantity()?;
        order_decoder.read_order_type()?;
        order_decoder.read_limit_type()?;
        order_decoder.read_aux_price()?;
        order_decoder.read_tif()?;
        order_decoder.read_oca_group()?;
        order_decoder.read_account()?;
        order_decoder.read_open_close()?;
        order_decoder.read_origin()?;
        order_decoder.read_order_ref()?;
        order_decoder.read_client_id()?;
        order_decoder.read_perm_id()?;
        order_decoder.read_outside_rth()?;
        order_decoder.read_hidden()?;
        order_decoder.read_discretionary_amount()?;
        order_decoder.read_good_after_time()?;
        order_decoder.skip_shares_allocation()?;
        order_decoder.read_faparams()?;
        order_decoder.read_model_code()?;
        order_decoder.read_good_till_date()?;
        order_decoder.read_rule80a()?;
        order_decoder.read_percent_offset()?;
        order_decoder.read_settling_firm()?;
        order_decoder.read_short_sale_params()?;
        order_decoder.read_auction_strategy()?;
        order_decoder.read_box_order_params()?;
        order_decoder.read_peg_to_stk_or_vol_order_params()?;
        order_decoder.read_display_size()?;
        order_decoder.read_old_style_outside_rth()?;
        order_decoder.read_block_order()?;
        order_decoder.read_sweep_to_fill()?;
        order_decoder.read_all_or_none()?;
        order_decoder.read_min_qty()?;
        order_decoder.read_oca_type()?;
        order_decoder.read_etrade_only()?;
        order_decoder.read_firm_quote_only()?;
        order_decoder.read_nbbo_price_cap()?;
        order_decoder.read_parent_id()?;
        order_decoder.read_trigger_method()?;
        order_decoder.read_vol_order_params(true)?;
        order_decoder.read_trail_params()?;
        order_decoder.read_basis_points()?;
        order_decoder.read_combo_legs()?;
        order_decoder.read_smart_combo_routing_params()?;
        order_decoder.read_scale_order_params()?;
        order_decoder.read_hedge_params()?;
        order_decoder.read_opt_out_smart_routing()?;
        order_decoder.read_clearing_params()?;
        order_decoder.read_not_held()?;
        order_decoder.read_delta_neutral()?;
        order_decoder.read_algo_params()?;
        order_decoder.read_solicited()?;
        order_decoder.read_what_if_info_and_commission()?;
        order_decoder.read_vol_randomize_flags()?;
        order_decoder.read_peg_to_bench_params()?;
        order_decoder.read_conditions()?;
        order_decoder.read_adjusted_order_params()?;
        order_decoder.read_soft_dollar_tier()?;
        order_decoder.read_cash_qty()?;
        order_decoder.read_dont_use_auto_price_for_hedge()?;
        order_decoder.read_is_oms_container()?;
        order_decoder.read_discretionary_up_to_limit_price()?;
        order_decoder.read_use_price_mgmt_algo()?;

        let order_data_item = OrderDataItem::new(order.order_id, order, contract, order_state);
        Ok(order_data_item)
    }

    fn contract_data_event(&mut self, server_version: i32) -> Result<(i32, ContractDetails), Box<dyn Error>> {
        let mut contract = ContractDetails::new();

        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();

        let version: i32 = decode_i32(&mut fields_itr)?;

        let mut req_id = -1;
        if version >= 3 {
            req_id = decode_i32(&mut fields_itr)?;
        }

        contract.contract.symbol = decode_string(&mut fields_itr)?;
        contract.contract.sec_type = decode_string(&mut fields_itr)?;
        self.read_last_trade_date(&mut contract, false, &mut fields_itr)?;
        contract.contract.strike = decode_f64(&mut fields_itr)?;
        contract.contract.right = decode_string(&mut fields_itr)?;
        contract.contract.exchange = decode_string(&mut fields_itr)?;
        contract.contract.currency = decode_string(&mut fields_itr)?;
        contract.contract.local_symbol = decode_string(&mut fields_itr)?;
        contract.market_name = decode_string(&mut fields_itr)?;
        contract.contract.trading_class = decode_string(&mut fields_itr)?;
        contract.contract.con_id = decode_i32(&mut fields_itr)?;
        contract.min_tick = decode_f64(&mut fields_itr)?;
        if server_version >= min_server_version::MD_SIZE_MULTIPLIER {
            contract.md_size_multiplier = decode_i32(&mut fields_itr)?;
        }
        contract.contract.multiplier = decode_string(&mut fields_itr)?;
        contract.order_types = decode_string(&mut fields_itr)?;
        contract.valid_exchanges = decode_string(&mut fields_itr)?;
        contract.price_magnifier = decode_i32(&mut fields_itr)?;
        if version >= 4 {
            contract.under_con_id = decode_i32(&mut fields_itr)?;
        }
        if version >= 5 {
            contract.long_name = decode_string(&mut fields_itr)?;
            contract.contract.primary_exchange = decode_string(&mut fields_itr)?;
        }

        if version >= 6 {
            contract.contract_month = decode_string(&mut fields_itr)?;
            contract.industry = decode_string(&mut fields_itr)?;
            contract.category = decode_string(&mut fields_itr)?;
            contract.subcategory = decode_string(&mut fields_itr)?;
            contract.time_zone_id = decode_string(&mut fields_itr)?;
            contract.trading_hours = decode_string(&mut fields_itr)?;
            contract.liquid_hours = decode_string(&mut fields_itr)?;
        }
        if version >= 8 {
            contract.ev_rule = decode_string(&mut fields_itr)?;
            contract.ev_multiplier = decode_f64(&mut fields_itr)?;
        }

        if version >= 7 {
            let sec_id_list_count = decode_i32(&mut fields_itr)?;
            if sec_id_list_count > 0 {
                contract.sec_id_list = vec![];
                for _ in 0..sec_id_list_count {
                    contract.sec_id_list.push(TagValue::new(
                        decode_string(&mut fields_itr)?,
                        decode_string(&mut fields_itr)?,
                    ));
                }
            }
        }

        if server_version >= min_server_version::AGG_GROUP {
            contract.agg_group = decode_i32(&mut fields_itr)?;
        }

        if server_version >= min_server_version::UNDERLYING_INFO {
            contract.under_symbol = decode_string(&mut fields_itr)?;
            contract.under_sec_type = decode_string(&mut fields_itr)?;
        }
        if server_version >= min_server_version::MARKET_RULES {
            contract.market_rule_ids = decode_string(&mut fields_itr)?;
        }

        if server_version >= min_server_version::REAL_EXPIRATION_DATE {
            contract.real_expiration_date = decode_string(&mut fields_itr)?;
        }

        Ok((req_id, contract))
    }

    fn read_last_trade_date(&self, contract: &mut ContractDetails, is_bond: bool, fields: &mut Iter<String>) -> Result<(), Box<dyn Error>> {
        let last_trade_date_or_contract_month = decode_string(fields)?;
        if last_trade_date_or_contract_month.len() > 0
        {
            let re = regex::Regex::new(r"\s+").unwrap();
            let mut count = 0;
            for part in re.split(last_trade_date_or_contract_month.as_ref()) {
                if count == 0
                {
                    if is_bond
                    {
                        contract.maturity = part.parse().unwrap();
                    } else {
                        contract.contract.last_trade_date_or_contract_month = part.parse().unwrap();
                    }
                }
                if count ==  1
                {
                    contract.last_trade_time = part.parse().unwrap();
                }
                if is_bond && count == 2
                {
                    contract.time_zone_id = part.parse().unwrap();
                }
                count += 1;
            }
        }

        Ok(())
    }

    fn order_status_event(&mut self, server_version: i32) -> Result<OrderStatusMessage, Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();

        if server_version < min_server_version::MARKET_CAP_PRICE {
            fields_itr.next();
        }

        let order_id = decode_i32(&mut fields_itr)?;

        let status = decode_string(&mut fields_itr)?;

        let filled;
        if server_version >= min_server_version::FRACTIONAL_POSITIONS {
            filled = decode_f64(&mut fields_itr)?;
        } else {
            filled = decode_i32(&mut fields_itr)? as f64;
        }

        let remaining;

        if server_version >= min_server_version::FRACTIONAL_POSITIONS {
            remaining = decode_f64(&mut fields_itr)?;
        } else {
            remaining = decode_i32(&mut fields_itr)? as f64;
        }

        let avg_fill_price = decode_f64(&mut fields_itr)?;

        let perm_id = decode_i32(&mut fields_itr)?; // ver 2 field
        let parent_id = decode_i32(&mut fields_itr)?; // ver 3 field
        let last_fill_price = decode_f64(&mut fields_itr)?; // ver 4 field
        let client_id = decode_i32(&mut fields_itr)?; // ver 5 field
        let why_held = decode_string(&mut fields_itr)?; // ver 6 field

        let mut mkt_cap_price = 0.0;
        if server_version >= min_server_version::MARKET_CAP_PRICE {
            mkt_cap_price = decode_f64(&mut fields_itr)?;
        }

        let order_status_message = OrderStatusMessage {
            order_id,
            status,
            filled,
            remaining,
            avg_fill_price,
            perm_id,
            parent_id,
            last_fill_price,
            client_id,
            why_held,
            mkt_cap_price
        };

        Ok(order_status_message)
    }

    fn historical_data_event(&mut self, server_version: i32, sender: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();

        if server_version < min_server_version::SYNT_REALTIME_BARS {
            fields_itr.next();
        }

        let req_id = decode_i32(&mut fields_itr)?;
        let start_date = decode_string(&mut fields_itr)?; // ver 2 field
        let end_date = decode_string(&mut fields_itr)?; // ver 2 field

        let _peek = *(fields_itr.clone()).peekable().peek().unwrap();

        let bar_count = decode_i32(&mut fields_itr)?;

        for _ in 0..bar_count {
            let date = decode_string(&mut fields_itr)?;
            let open = decode_f64(&mut fields_itr)?;
            let high = decode_f64(&mut fields_itr)?;
            let low = decode_f64(&mut fields_itr)?;
            let close = decode_f64(&mut fields_itr)?;
            let volume = if server_version < min_server_version::SYNT_REALTIME_BARS {
                decode_i32(&mut fields_itr)? as i64
            } else {
                decode_i64(&mut fields_itr)?
            };
            let average = decode_f64(&mut fields_itr)?;

            if server_version < min_server_version::SYNT_REALTIME_BARS {
                decode_string(&mut fields_itr)?; //has_gaps
            }

            let bar_count = decode_i32(&mut fields_itr)?; // ver 3 field

            sender.send(IncomingMessagesEnum::HistoricalData(Bar {
                time_str: date,
                time_int: 0,
                open,
                high,
                low,
                close,
                volume: volume as f64,
                wap: average,
                count: bar_count as f64,
                color: None
            }))?;
        }

        sender.send(IncomingMessagesEnum::HistoricalDataEnd(req_id, start_date, end_date))?;

        Ok(())
    }

    fn read_time_bars_event(&mut self, sender: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let req_id = decode_i32(&mut fields_itr)?;

        let bar = Bar {
            time_str: decode_string(&mut fields_itr)?,
            time_int: 0,
            open: decode_f64(&mut fields_itr)?,
            high: decode_f64(&mut fields_itr)?,
            low: decode_f64(&mut fields_itr)?,
            close: decode_f64(&mut fields_itr)?,
            volume: decode_i64(&mut fields_itr)? as f64,
            wap: decode_f64(&mut fields_itr)?,
            count: decode_i32(&mut fields_itr)? as f64,
            color: None
        };

        sender.send(IncomingMessagesEnum::RealTimeBars(req_id, bar))?;

        Ok(())
    }

    fn tick_by_tick_event(&mut self, sender: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();

        let req_id = decode_i32(&mut fields_itr)?;

        let tick_type = decode_i32(&mut fields_itr)?;
        let time = decode_i64(&mut fields_itr)?;

        match tick_type {
            0 => return Ok(()), // None
            1..=2 =>
            // Last (1) or AllLast (2)
                {
                    let price = decode_f64(&mut fields_itr)?;
                    let size = decode_i32(&mut fields_itr)?;
                    let mask = decode_i32(&mut fields_itr)?;
//                    let mut tick_attrib_last = TickAttribLast::default();
//                    tick_attrib_last.past_limit = mask & 1 != 0;
//                    tick_attrib_last.unreported = mask & 2 != 0;
                    let exchange = decode_string(&mut fields_itr)?;
                    let special_conditions = decode_string(&mut fields_itr)?;

                    let tick_last = TickLast {
                        time,
                        price,
                        size,
                        mask,
                        exchange,
                        special_conditions
                    };

                    sender.send(IncomingMessagesEnum::TickByTickLast((req_id, tick_last)))?;
                }
            3 =>
            // BidAsk
                {
                    // let bid_price = decode_f64(&mut fields_itr)?;
                    // let ask_price = decode_f64(&mut fields_itr)?;
                    // let bid_size = decode_i32(&mut fields_itr)?;
                    // let ask_size = decode_i32(&mut fields_itr)?;
                    // let mask = decode_i32(&mut fields_itr)?;
//                    let mut tick_attrib_bid_ask = TickAttribBidAsk::default();
//                    tick_attrib_bid_ask.bid_past_low = mask & 1 != 0;
//                    tick_attrib_bid_ask.ask_past_high = mask & 2 != 0;
//                    self.wrapper
//                        .lock()
//                        .expect(WRAPPER_POISONED_MUTEX)
//                        .tick_by_tick_bid_ask(
//                            req_id,
//                            time,
//                            bid_price,
//                            ask_price,
//                            bid_size,
//                            ask_size,
//                            tick_attrib_bid_ask,
//                        );
                }
            4 =>
            // MidPoint
                {
                    // let mid_point = decode_f64(&mut fields_itr)?;
//                    self.wrapper
//                        .lock()
//                        .expect(WRAPPER_POISONED_MUTEX)
//                        .tick_by_tick_mid_point(req_id, time, mid_point);
                }
            _ => return Ok(()),
        }
        Ok(())
    }

    fn generic_tick(&mut self, sender: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();
        //throw away version
        fields_itr.next();

        let req_id = decode_i32(&mut fields_itr)?;
        let tick_type = decode_i32(&mut fields_itr)?;
        let value = decode_f64(&mut fields_itr)?;

        sender.send(IncomingMessagesEnum::TickGeneric(req_id, tick_type, value))?;

        Ok(())
    }

    fn profit_and_losses(&mut self, server_version: i32, sender: &Sender<IncomingMessagesEnum>) -> Result<(), Box<dyn Error>> {
        let mut fields_itr = self.fields.iter();

        //throw away message_id
        fields_itr.next();

        let req_id = decode_i32(&mut fields_itr)?;
        let daily_pnl = decode_f64(&mut fields_itr)?;
        let mut unrealized_pnl = 0.0;
        let mut realized_pnl = 0.0;

        if server_version >= min_server_version::UNREALIZED_PNL {
            unrealized_pnl = decode_f64(&mut fields_itr)?;
        }

        if server_version >= min_server_version::REALIZED_PNL {
            realized_pnl = decode_f64(&mut fields_itr)?;
        }

        sender.send(IncomingMessagesEnum::PnL(req_id, daily_pnl, unrealized_pnl, realized_pnl))?;

        Ok(())
    }
}

