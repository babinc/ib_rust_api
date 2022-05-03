use crate::decoder::{decode_i32, decode_string, decode_f64, decode_f64_show_unset, decode_bool, decode_i32_show_unset};
use crate::models::contract::Contract;
use crate::models::order::Order;
use crate::models::order_state::OrderState;
use crate::constants::min_server_version;
use crate::enums::origin::Origin;
use crate::models::combo_leg::ComboLeg;
use crate::models::order_combo_leg::OrderComboLeg;
use crate::models::tag_value::TagValue;
use crate::models::delta_neutral_contract::DeltaNeutralContract;
use crate::models::soft_dollar_tier::SoftDollarTier;
use crate::enums::order_condition_type::OrderConditionType;
use crate::enums::position_type::PositionType;
use crate::traits::order_condition::OrderCondition;
use crate::models::conditions::price_condition::PriceCondition;
use crate::models::conditions::execution_condition::ExecutionCondition;
use crate::models::conditions::time_condition::TimeCondition;
use crate::models::conditions::margin_condition::MarginCondition;
use crate::models::conditions::volume_condition::VolumeCondition;
use crate::models::conditions::percent_change_condition::PercentChangeCondition;
use std::slice::Iter;
use std::error::Error;

pub struct OrderDecoder<'a> {
    contract: &'a mut Contract,
    order: &'a mut Order,
    order_state: &'a mut OrderState,
    message_version: i32,
    server_version: i32,
    fields_iter: Iter<'a, String>
}

impl OrderDecoder<'_> {
    pub fn new<'a>(contract: &'a mut Contract, order: &'a mut Order, order_state: &'a mut OrderState, message_version: i32, server_version: i32, fields_iter: Iter<'a, String>) -> OrderDecoder<'a> {
        OrderDecoder {
            contract,
            order,
            order_state,
            message_version,
            server_version,
            fields_iter,
        }
    }

    pub fn read_order_id(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.order_id = decode_i32(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_contract_fields(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 17 {
            self.contract.con_id = decode_i32(&mut self.fields_iter)?;
        }
        self.contract.symbol = decode_string(&mut self.fields_iter)?;
        self.contract.sec_type = decode_string(&mut self.fields_iter)?;
        self.contract.last_trade_date_or_contract_month = decode_string(&mut self.fields_iter)?;
        self.contract.strike = decode_f64(&mut self.fields_iter)?;
        self.contract.right = decode_string(&mut self.fields_iter)?;
        if self.message_version >= 32 {
            self.contract.multiplier = decode_string(&mut self.fields_iter)?;
        }
        self.contract.exchange = decode_string(&mut self.fields_iter)?;
        self.contract.currency = decode_string(&mut self.fields_iter)?;
        if self.message_version >= 2 {
            self.contract.local_symbol = decode_string(&mut self.fields_iter)?;
        }
        if self.message_version >= 32 {
            self.contract.trading_class = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_action(&mut self) -> Result<(), Box<dyn Error>> {
       self.order.action = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_total_quantity(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::FRACTIONAL_POSITIONS {
            self.order.total_quantity = decode_f64(&mut self.fields_iter)?;
        }
        else {
            self.order.total_quantity = decode_i32(&mut self.fields_iter)? as f64;
        }
        Ok(())
    }

    pub fn read_order_type(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.order_type = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_limit_type(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version < 29 {
            self.order.lmt_price = decode_f64(&mut self.fields_iter)?;
        }
        else {
            self.order.lmt_price = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_aux_price(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version < 30 {
            self.order.aux_price = decode_f64(&mut self.fields_iter)?;
        }
        else {
            self.order.aux_price = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_tif(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.tif = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_oca_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.oca_group = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_account(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.account = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_open_close(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.open_close = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_origin(&mut self) -> Result<(), Box<dyn Error>> {
        let res = decode_i32(&mut self.fields_iter)?;
        match res {
            0 => self.order.origin = Origin::Customer,
            1 => self.order.origin = Origin::Firm,
            _ => self.order.origin = Origin::Unknown
        }
        Ok(())
    }

    pub fn read_order_ref(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.order_ref = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_client_id(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 3 {
            self.order.client_id = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_perm_id(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 4 {
            self.order.perm_id = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_outside_rth(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 4 {
            if self.message_version < 18 {
                //will never happen
            }
            else {
                self.order.outside_rth = decode_bool(&mut self.fields_iter)?;
            }
        }
        Ok(())
    }

    pub fn read_hidden(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 4 {
            self.order.hidden = decode_i32(&mut self.fields_iter)? == 1;
        }
        Ok(())
    }

    pub fn read_discretionary_amount(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 4 {
            self.order.discretionary_amt = decode_f64(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_good_after_time(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 5 {
            self.order.good_after_time = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn skip_shares_allocation(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 6 {
            //skip deprecated shares field
            decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_faparams(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 7
        {
            self.order.fa_group = decode_string(&mut self.fields_iter)?;
            self.order.fa_method = decode_string(&mut self.fields_iter)?;
            self.order.fa_percentage = decode_string(&mut self.fields_iter)?;
            self.order.fa_profile = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_model_code(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::MODELS_SUPPORT {
            self.order.model_code = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_good_till_date(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 8 {
            self.order.good_till_date = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_rule80a(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.rule80a = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_percent_offset(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.percent_offset = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_settling_firm(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.settling_firm = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_short_sale_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.short_sale_slot = decode_i32(&mut self.fields_iter)?;
            self.order.designated_location = decode_string(&mut self.fields_iter)?;
            if self.server_version == 51 {
                decode_i32(&mut self.fields_iter)?;
            }
            else if self.message_version >= 23 {
                self.order.exempt_code = decode_i32(&mut self.fields_iter)?;
            }
        }

        Ok(())
    }

    pub fn read_auction_strategy(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.auction_strategy = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_box_order_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.starting_price = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.stock_ref_price = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.delta = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_peg_to_stk_or_vol_order_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.stock_range_lower = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.stock_range_upper = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_display_size(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.display_size = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_old_style_outside_rth(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9
        {
            if self.message_version < 18 {
                // will never happen
                /* order.rthOnly = */
                decode_bool(&mut self.fields_iter)?;
            }
        }
        Ok(())
    }

    pub fn read_block_order(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.block_order = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_sweep_to_fill(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.sweep_to_fill = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_all_or_none(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.all_or_none = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_min_qty(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.min_qty = decode_i32_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_oca_type(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.oca_type = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_etrade_only(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.e_trade_only = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_firm_quote_only(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.firm_quote_only = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_nbbo_price_cap(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 9 {
            self.order.nbbo_price_cap = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_parent_id(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 10
        {
            self.order.parent_id = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_trigger_method(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 10 {
            self.order.trigger_method = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_vol_order_params(&mut self, read_open_order_attribs: bool) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 11 {
            self.order.volatility = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.volatility_type = decode_i32(&mut self.fields_iter)?;
            if self.message_version == 11 {
                let received_int = decode_i32(&mut self.fields_iter)?;
                self.order.delta_neutral_order_type = match received_int {
                    0 => "NONE".to_string(),
                    _ => "MKT".to_string()
                }
            }
            else { // msgVersion 12 and up
                self.order.delta_neutral_order_type = decode_string(&mut self.fields_iter)?;
                self.order.delta_neutral_aux_price = decode_f64_show_unset(&mut self.fields_iter)?;

                if self.message_version >= 27 && self.order.delta_neutral_order_type.is_empty() == false
                {
                    self.order.delta_neutral_con_id = decode_i32(&mut self.fields_iter)?;
                    if read_open_order_attribs
                    {
                        self.order.delta_neutral_settling_firm = decode_string(&mut self.fields_iter)?;
                        self.order.delta_neutral_clearing_account = decode_string(&mut self.fields_iter)?;
                        self.order.delta_neutral_clearing_intent = decode_string(&mut self.fields_iter)?;
                    }
                }

                if self.message_version >= 31 && self.order.delta_neutral_order_type.is_empty() == false
                {
                    if read_open_order_attribs
                    {
                        self.order.delta_neutral_open_close = decode_string(&mut self.fields_iter)?;
                    }
                    self.order.delta_neutral_short_sale = decode_bool(&mut self.fields_iter)?;
                    self.order.delta_neutral_short_sale_slot = decode_i32(&mut self.fields_iter)?;
                    self.order.delta_neutral_designated_location = decode_string(&mut self.fields_iter)?;
                }
            }
            self.order.continuous_update = decode_i32(&mut self.fields_iter)?;
            if self.server_version == 26 {
                self.order.stock_range_lower = decode_f64(&mut self.fields_iter)?;
                self.order.stock_range_upper = decode_f64(&mut self.fields_iter)?;
            }
            self.order.reference_price_type = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_trail_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 13 {
            self.order.trail_stop_price = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        if self.message_version >= 30 {
            self.order.trailing_percent = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_basis_points(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 14 {
            self.order.basis_points = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.basis_points_type = decode_i32_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_combo_legs(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 14 {
            self.contract.combo_legs_descrip = decode_string(&mut self.fields_iter)?;
        }

        if self.message_version >= 29 {
            let combo_legs_count = decode_i32(&mut self.fields_iter)?;
            if combo_legs_count > 0
            {
                for _ in 0..combo_legs_count
                {
                    let con_id = decode_i32(&mut self.fields_iter)?;
                    let ratio = decode_i32(&mut self.fields_iter)?;
                    let action = decode_string(&mut self.fields_iter)?;
                    let exchange = decode_string(&mut self.fields_iter)?;
                    let open_close = match decode_i32(&mut self.fields_iter)? {
                        0 => PositionType::SamePos,
                        1 => PositionType::OpenPos,
                        2 => PositionType::ClosePos,
                        _ => PositionType::UnknownPos
                    };
                    let short_sale_slot = decode_i32(&mut self.fields_iter)?;
                    let designated_location = decode_string(&mut self.fields_iter)?;
                    let exempt_code = decode_i32(&mut self.fields_iter)?;

                    let combo_leg: ComboLeg = ComboLeg::new(con_id, ratio, action, exchange, open_close,
                                                           short_sale_slot, designated_location, exempt_code);
                    self.contract.combo_legs.push(combo_leg);
                }
            }

            let order_combo_legs_count = decode_i32(&mut self.fields_iter)?;
            if order_combo_legs_count > 0
            {
                for _ in 0..order_combo_legs_count
                {
                    let price = decode_f64_show_unset(&mut self.fields_iter)?;

                    let order_combo_leg: OrderComboLeg = OrderComboLeg::new(price);
                    self.order.order_combo_legs.push(order_combo_leg);
                }
            }
        }
        Ok(())
    }

    pub fn read_smart_combo_routing_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 26 {
            let smart_combo_routing_params_count = decode_i32(&mut self.fields_iter)?;
            if smart_combo_routing_params_count > 0
            {
                for _ in 0..smart_combo_routing_params_count
                {
                    let tag = decode_string(&mut self.fields_iter)?;
                    let value = decode_string(&mut self.fields_iter)?;
                    self.order.smart_combo_routing_params.push(TagValue::new(tag, value));
                }
            }
        }
        Ok(())
    }

    pub fn read_scale_order_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 15 {
            if self.message_version >= 20 {
                self.order.scale_init_level_size = decode_i32_show_unset(&mut self.fields_iter)?;
                self.order.scale_subs_level_size = decode_i32_show_unset(&mut self.fields_iter)?;
            }
            else {
                /* int notSuppScaleNumComponents = */
                decode_i32_show_unset(&mut self.fields_iter)?;
                self.order.scale_init_level_size = decode_i32_show_unset(&mut self.fields_iter)?;
            }
            self.order.scale_price_increment = decode_f64_show_unset(&mut self.fields_iter)?;
        }

        if self.message_version >= 28 && self.order.scale_price_increment > 0.0 && self.order.scale_price_increment != f64::MAX {
            self.order.scale_price_adjust_value = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.scale_price_adjust_interval = decode_i32_show_unset(&mut self.fields_iter)?;
            self.order.scale_profit_offset = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.scale_auto_reset = decode_bool(&mut self.fields_iter)?;
            self.order.scale_init_position = decode_i32_show_unset(&mut self.fields_iter)?;
            self.order.scale_init_fill_qty = decode_i32_show_unset(&mut self.fields_iter)?;
            self.order.scale_random_percent = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_hedge_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 24 {
            self.order.hedge_type = decode_string(&mut self.fields_iter)?;
            if self.order.hedge_type.is_empty() == false {
                self.order.hedge_param = decode_string(&mut self.fields_iter)?;
            }
        }
        Ok(())
    }

    pub fn read_opt_out_smart_routing(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 25 {
            self.order.opt_out_smart_routing = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_clearing_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 19 {
            self.order.clearing_account = decode_string(&mut self.fields_iter)?;
            self.order.clearing_intent = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_not_held(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 22 {
            self.order.not_held = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_delta_neutral(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 20 {
            if decode_bool(&mut self.fields_iter)?
            {
                let con_id = decode_i32(&mut self.fields_iter)?;
                let delta = decode_f64(&mut self.fields_iter)?;
                let price = decode_f64(&mut self.fields_iter)?;
                self.contract.delta_neutral_contract = Some(DeltaNeutralContract::new(con_id, delta, price));
            }
        }
        Ok(())
    }

    pub fn read_algo_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 21 {
            self.order.algo_strategy = decode_string(&mut self.fields_iter)?;
            if self.order.algo_strategy.is_empty() == false
            {
                let algo_params_count = decode_i32(&mut self.fields_iter)?;
                if algo_params_count > 0
                {
                    for _ in 0.. algo_params_count
                    {
                        let tag = decode_string(&mut self.fields_iter)?;
                        let value = decode_string(&mut self.fields_iter)?;
                        self.order.algo_params.push(TagValue::new(tag, value));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn read_solicited(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 33 {
            self.order.solicited = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_what_if_info_and_commission(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 16 {
            self.order.what_if = decode_bool(&mut self.fields_iter)?;
            self.read_order_status()?;
            if self.server_version >= min_server_version::WHAT_IF_EXT_FIELDS {
                self.order_state.init_margin_before = decode_string(&mut self.fields_iter)?;
                self.order_state.maint_margin_before = decode_string(&mut self.fields_iter)?;
                self.order_state.equity_with_loan_before = decode_string(&mut self.fields_iter)?;
                self.order_state.init_margin_change = decode_string(&mut self.fields_iter)?;
                self.order_state.maint_margin_change = decode_string(&mut self.fields_iter)?;
                self.order_state.equity_with_loan_change = decode_string(&mut self.fields_iter)?;
            }
            self.order_state.init_margin_after = decode_string(&mut self.fields_iter)?;
            self.order_state.maint_margin_after = decode_string(&mut self.fields_iter)?;
            self.order_state.equity_with_loan_after = decode_string(&mut self.fields_iter)?;
            self.order_state.commission = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order_state.min_commission = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order_state.max_commission = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order_state.commission_currency = decode_string(&mut self.fields_iter)?;
            self.order_state.warning_text = decode_string(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_order_status(&mut self) -> Result<(), Box<dyn Error>> {
        self.order_state.status = decode_string(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_vol_randomize_flags(&mut self) -> Result<(), Box<dyn Error>> {
        if self.message_version >= 34 {
            self.order.randomize_size = decode_bool(&mut self.fields_iter)?;
            self.order.randomize_price = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_peg_to_bench_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::PEGGED_TO_BENCHMARK {
            if self.order.order_type == "PEG BENCH".to_string() {
                self.order.reference_contract_id = decode_i32(&mut self.fields_iter)?;
                self.order.is_pegged_change_amount_decrease = decode_bool(&mut self.fields_iter)?;
                self.order.pegged_change_amount = decode_f64_show_unset(&mut self.fields_iter)?;
                self.order.reference_change_amount = decode_f64_show_unset(&mut self.fields_iter)?;
                self.order.reference_exchange = decode_string(&mut self.fields_iter)?;
            }
        }
        Ok(())
    }

    pub fn read_conditions(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::PEGGED_TO_BENCHMARK {
            let n_conditions = decode_i32(&mut self.fields_iter)?;

            if n_conditions > 0 {
                for _ in 0..n_conditions {
                    let condition_num = decode_i32(&mut self.fields_iter)?;
                    let order_condition_type: OrderConditionType = OrderConditionType::from(condition_num);

                    let mut order_condition: Box<dyn OrderCondition> = match order_condition_type {
                        OrderConditionType::Price => Box::new(PriceCondition::new()),
                        OrderConditionType::Execution => Box::new(ExecutionCondition::new()),
                        OrderConditionType::Time => Box::new(TimeCondition::new()),
                        OrderConditionType::Margin => Box::new(MarginCondition::new()),
                        OrderConditionType::Volume => Box::new(VolumeCondition::new()),
                        OrderConditionType::PercentChange => Box::new(PercentChangeCondition::new()),
                        _ => Box::new(PriceCondition::new()), //TODO print error
                    };

                    order_condition.deserialize(&mut self.fields_iter);
                    self.order.conditions.push(order_condition);
                }

                self.order.conditions_ignore_rth = decode_bool(&mut self.fields_iter)?;
                self.order.conditions_cancel_order = decode_bool(&mut self.fields_iter)?;
            }
        }
        Ok(())
    }

    pub fn read_adjusted_order_params(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::PEGGED_TO_BENCHMARK {
            self.order.adjusted_order_type = decode_string(&mut self.fields_iter)?;
            self.order.trigger_price = decode_f64_show_unset(&mut self.fields_iter)?;
            self.read_stop_price_and_lmt_price_offset()?;
            self.order.adjusted_stop_price = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.adjusted_stop_limit_price = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.adjusted_trailing_amount = decode_f64_show_unset(&mut self.fields_iter)?;
            self.order.adjustable_trailing_unit = decode_i32(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_stop_price_and_lmt_price_offset(&mut self) -> Result<(), Box<dyn Error>> {
        self.order.trail_stop_price = decode_f64_show_unset(&mut self.fields_iter)?;
        self.order.lmt_price_offset = decode_f64_show_unset(&mut self.fields_iter)?;
        Ok(())
    }

    pub fn read_soft_dollar_tier(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::SOFT_DOLLAR_TIER {
            self.order.soft_dollar_tier = SoftDollarTier::new(decode_string(&mut self.fields_iter)?,
                                                 decode_string(&mut self.fields_iter)?,
                                                 decode_string(&mut self.fields_iter)?);
        }
        Ok(())
    }

    pub fn read_cash_qty(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::CASH_QTY {
            self.order.cash_qty = decode_f64_show_unset(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_dont_use_auto_price_for_hedge(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::AUTO_PRICE_FOR_HEDGE {
            self.order.dont_use_auto_price_for_hedge = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_is_oms_container(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::ORDER_CONTAINER {
            self.order.is_oms_container = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    pub fn read_discretionary_up_to_limit_price(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::D_PEG_ORDERS {
            self.order.discretionary_up_to_limit_price = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }

    // pub fn read_auto_cancel_date(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.auto_cancel_date = decode_string(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_filled_quantity(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.filled_quantity = decode_f64_show_unset(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_ref_futures_con_id(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.ref_futures_con_id = decode_i32(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_auto_cancel_parent(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.auto_cancel_parent = decode_bool(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_shareholder(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.shareholder = decode_string(&mut self.fields_iter)?;
    //     Ok(())
    // }

    // pub fn read_imbalance_only(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.imbalance_only = decode_bool(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_route_marketable_to_bbo(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.route_marketable_to_bbo = decode_bool(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_parent_perm_id(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order.parent_perm_id = decode_i32(&mut self.fields_iter)? as i64;
    //     Ok(())
    // }
    //
    // pub fn read_completed_time(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order_state.completed_time = decode_string(&mut self.fields_iter)?;
    //     Ok(())
    // }
    //
    // pub fn read_completed_status(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.order_state.completed_status = decode_string(&mut self.fields_iter)?;
    //     Ok(())
    // }

    pub fn read_use_price_mgmt_algo(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_version >= min_server_version::PRICE_MGMT_ALGO {
            self.order.use_price_mgmt_algo = decode_bool(&mut self.fields_iter)?;
        }
        Ok(())
    }
}