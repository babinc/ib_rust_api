use crate::models::soft_dollar_tier::SoftDollarTier;
use crate::enums::origin::Origin;
use crate::models::tag_value::TagValue;
use crate::models::order_combo_leg::OrderComboLeg;
use crate::traits::order_condition::OrderCondition;

pub struct Order {
    pub soft_dollar_tier: SoftDollarTier,
    pub order_id: i32,
    pub client_id: i32,
    pub perm_id: i32,
    pub action: String,
    pub total_quantity: f64,
    pub order_type: String,
    pub lmt_price: f64,
    pub aux_price: f64,
    pub tif: String,
    pub active_start_time: String,
    pub active_stop_time: String,
    pub oca_group: String,
    pub oca_type: i32,
    pub order_ref: String,
    pub transmit: bool,
    pub parent_id: i32,
    pub block_order: bool,
    pub sweep_to_fill: bool,
    pub display_size: i32,
    pub trigger_method: i32,
    // 0=Default, 1=Double_Bid_Ask, 2=Last, 3=Double_Last, 4=Bid_Ask, 7=Last_or_Bid_Ask, 8=Mid-point
    pub outside_rth: bool,
    pub hidden: bool,
    pub good_after_time: String,
    // Format: 20060505 08:00:00 {time zone}
    pub good_till_date: String,
    // Format: 20060505 08:00:00 {time zone}
    pub rule80a: String,
    // Individual = 'I', Agency = 'A', AgentOtherMember = 'W', IndividualPTIA = 'J', AgencyPTIA = 'U', AgentOtherMemberPTIA = 'M', IndividualPT = 'K', AgencyPT = 'Y', AgentOtherMemberPT = 'N'
    pub all_or_none: bool,
    pub min_qty: i32,
    pub percent_offset: f64,
    pub override_percentage_constraints: bool,
    pub trail_stop_price: f64,
    pub trailing_percent: f64,

    // financial advisers only
    pub fa_group: String,
    pub fa_profile: String,
    pub fa_method: String,
    pub fa_percentage: String,

    // institutional (ie non-cleared) only
    pub designated_location: String,
    //used only when short_sale_slot=2
    pub open_close: String,
    // O=Open, C=Close
    pub origin: Origin,
    // 0=Customer, 1=Firm
    pub short_sale_slot: i32,
    // type: int; 1 if you hold the shares, 2 if they will be delivered from elsewhere.  Only for Action=SSHORT
    pub exempt_code: i32,

    // SMART routing only
    pub discretionary_amt: f64,
    pub e_trade_only: bool,
    pub firm_quote_only: bool,
    pub nbbo_price_cap: f64,
    pub opt_out_smart_routing: bool,

    // BOX exchange orders only
    pub auction_strategy: i32,
    // type: int; AuctionMatch, AuctionImprovement, AuctionTransparent
    pub starting_price: f64,
    pub stock_ref_price: f64,
    pub delta: f64, // type: float

    // pegged to stock and VOL orders only
    pub stock_range_lower: f64,
    pub stock_range_upper: f64,

    pub randomize_price: bool,
    pub randomize_size: bool,

    // VOLATILITY ORDERS ONLY
    pub volatility: f64,
    pub volatility_type: i32,
    // type: int   // 1=daily, 2=annual
    pub delta_neutral_order_type: String,
    pub delta_neutral_aux_price: f64,
    pub delta_neutral_con_id: i32,
    pub delta_neutral_settling_firm: String,
    pub delta_neutral_clearing_account: String,
    pub delta_neutral_clearing_intent: String,
    pub delta_neutral_open_close: String,
    pub delta_neutral_short_sale: bool,
    pub delta_neutral_short_sale_slot: i32,
    pub delta_neutral_designated_location: String,
    pub continuous_update: i32,
    pub reference_price_type: i32, // type: int; 1=Average, 2 = BidOrAsk

    // COMBO ORDERS ONLY
    pub basis_points: f64,
    // type: float; EFP orders only
    pub basis_points_type: i32, // type: int;  EFP orders only

    // SCALE ORDERS ONLY
    pub scale_init_level_size: i32,
    pub scale_subs_level_size: i32,
    pub scale_price_increment: f64,
    pub scale_price_adjust_value: f64,
    pub scale_price_adjust_interval: i32,
    pub scale_profit_offset: f64,
    pub scale_auto_reset: bool,
    pub scale_init_position: i32,
    pub scale_init_fill_qty: i32,
    pub scale_random_percent: bool,
    pub scale_table: String,

    // HEDGE ORDERS
    pub hedge_type: String,
    // 'D' - delta, 'B' - beta, 'F' - FX, 'P' - pair
    pub hedge_param: String, // 'beta=X' value for beta hedge, 'ratio=Y' for pair hedge

    // Clearing info
    pub account: String,
    // IB account
    pub settling_firm: String,
    pub clearing_account: String,
    //True beneficiary of the order
    pub clearing_intent: String, // "" (Default), "IB", "Away", "PTA" (PostTrade)

    // ALGO ORDERS ONLY
    pub algo_strategy: String,

    pub algo_params: Vec<TagValue>,
    pub smart_combo_routing_params: Vec<TagValue>,
    pub algo_id: String,
    pub what_if: bool,
    pub not_held: bool,
    pub solicited: bool,
    pub model_code: String,
    pub order_combo_legs: Vec<OrderComboLeg>,
    pub order_misc_options: Vec<TagValue>,

    // VER PEG2BENCH fields:
    pub reference_contract_id: i32,
    pub pegged_change_amount: f64,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount: f64,
    pub reference_exchange: String,
    pub adjusted_order_type: String,

    pub trigger_price: f64,
    pub adjusted_stop_price: f64,
    pub adjusted_stop_limit_price: f64,
    pub adjusted_trailing_amount: f64,
    pub adjustable_trailing_unit: i32,
    pub lmt_price_offset: f64,

    pub conditions: Vec<Box<dyn OrderCondition>>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth: bool,

    pub ext_operator: String,
    pub cash_qty: f64,
    pub mifid2decision_maker: String,
    pub mifid2decision_algo: String,
    pub mifid2execution_trader: String,
    pub mifid2execution_algo: String,
    pub dont_use_auto_price_for_hedge: bool,
    pub is_oms_container: bool,
    pub discretionary_up_to_limit_price: bool,
    pub auto_cancel_date: String,
    pub filled_quantity: f64,
    pub ref_futures_con_id: i32,
    pub auto_cancel_parent: bool,
    pub shareholder: String,
    pub imbalance_only: bool,
    pub route_marketable_to_bbo: bool,
    pub parent_perm_id: i64,
    pub use_price_mgmt_algo: bool,
}

impl Order {
    pub fn new() -> Order {
        Order {
            soft_dollar_tier: SoftDollarTier::new("".to_string(), "".to_string(), "".to_string()),
            order_id: 0,
            client_id: 0,
            perm_id: 0,
            action: "".to_string(),
            total_quantity: 0.0,
            order_type: "".to_string(),
            lmt_price: f64::MAX,
            aux_price: f64::MAX,
            tif: "".to_string(),
            active_start_time: "".to_string(),
            active_stop_time: "".to_string(),
            oca_group: "".to_string(),
            oca_type: 0,
            order_ref: "".to_string(),
            transmit: true,
            parent_id: 0,
            block_order: false,
            sweep_to_fill: false,
            display_size: 0,
            trigger_method: 0,
            outside_rth: false,
            hidden: false,
            good_after_time: "".to_string(), // Format: 20060505 08:00:00 {time zone}
            good_till_date: "".to_string(),
            rule80a: "".to_string(),
            all_or_none: false,
            min_qty: std::i32::MAX,
            percent_offset: std::f64::MAX,
            override_percentage_constraints: false,
            trail_stop_price: std::f64::MAX,
            trailing_percent: std::f64::MAX,
            fa_group: "".to_string(),
            fa_profile: "".to_string(),
            fa_method: "".to_string(),
            fa_percentage: "".to_string(),
            designated_location: "".to_string(),
            open_close: "O".to_string(),
            origin: Origin::Customer,
            short_sale_slot: 0,
            exempt_code: -1,
            discretionary_amt: 0.0,
            e_trade_only: true,
            firm_quote_only: true,
            nbbo_price_cap: f64::MAX,
            opt_out_smart_routing: false,
            auction_strategy: 0,
            starting_price: f64::MAX,
            stock_ref_price: f64::MAX,
            delta: f64::MAX,
            stock_range_lower: f64::MAX,
            stock_range_upper: f64::MAX,
            randomize_price: false,
            randomize_size: false,
            volatility: f64::MAX,
            volatility_type: i32::MAX,
            continuous_update: 0,
            reference_price_type: i32::MAX,
            delta_neutral_order_type: "".to_string(),
            delta_neutral_aux_price: f64::MAX,
            delta_neutral_con_id: 0,
            delta_neutral_settling_firm: "".to_string(),
            delta_neutral_clearing_account: "".to_string(),
            delta_neutral_clearing_intent: "".to_string(),
            delta_neutral_open_close: "".to_string(),
            delta_neutral_short_sale: false,
            delta_neutral_short_sale_slot: 0,
            delta_neutral_designated_location: "".to_string(),
            basis_points: std::f64::MAX,
            basis_points_type: i32::MAX,
            scale_init_level_size: i32::MAX,
            scale_subs_level_size: i32::MAX,
            scale_price_increment: std::f64::MAX,
            scale_price_adjust_value: std::f64::MAX,
            scale_price_adjust_interval: i32::MAX,
            scale_profit_offset: std::f64::MAX,
            scale_auto_reset: false,
            scale_init_position: i32::MAX,
            scale_init_fill_qty: i32::MAX,
            scale_random_percent: false,
            scale_table: "".to_string(),
            hedge_type: "".to_string(),
            hedge_param: "".to_string(),
            account: "".to_string(),
            settling_firm: "".to_string(),
            clearing_account: "".to_string(),
            clearing_intent: "".to_string(),
            algo_strategy: "".to_string(),
            algo_params: vec![],
            smart_combo_routing_params: vec![],
            algo_id: "".to_string(),
            what_if: false,
            not_held: false,
            solicited: false,
            model_code: "".to_string(),
            order_combo_legs: vec![],
            order_misc_options: vec![],
            reference_contract_id: i32::MAX,
            pegged_change_amount: std::f64::MAX,
            is_pegged_change_amount_decrease: false,
            reference_change_amount: std::f64::MAX,
            reference_exchange: "".to_string(),
            adjusted_order_type: "".to_string(),
            trigger_price: std::f64::MAX,
            adjusted_stop_price: std::f64::MAX,
            adjusted_stop_limit_price: std::f64::MAX,
            adjusted_trailing_amount: std::f64::MAX,
            adjustable_trailing_unit: i32::MAX,
            lmt_price_offset: std::f64::MAX,
            conditions: vec![],
            conditions_cancel_order: false,
            conditions_ignore_rth: false,
            ext_operator: "".to_string(),
            cash_qty: f64::MAX,
            mifid2decision_maker: "".to_string(),
            mifid2decision_algo: "".to_string(),
            mifid2execution_trader: "".to_string(),
            mifid2execution_algo: "".to_string(),
            dont_use_auto_price_for_hedge: false,
            is_oms_container: false,
            discretionary_up_to_limit_price: false,
            auto_cancel_date: "".to_string(),
            filled_quantity: f64::MAX,
            ref_futures_con_id: i32::MAX,
            auto_cancel_parent: false,
            shareholder: "".to_string(),
            imbalance_only: false,
            route_marketable_to_bbo: false,
            parent_perm_id: i64::MAX,
            use_price_mgmt_algo: false
        }
    }
}