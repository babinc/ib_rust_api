use crate::models::scan_data_item::ScanDataItem;
use crate::models::account_summary::AccountSummary;
use crate::models::order_data_item::OrderDataItem;
use crate::models::contract_details::ContractDetails;
use crate::models::order_status_message::OrderStatusMessage;
use crate::models::bar::Bar;
use crate::models::tick_last::TickLast;

pub enum IncomingMessagesEnum {
    NotValid,
    TickPrice,
    TickSize,
    OrderStatus(OrderStatusMessage),
    Error(i32, i32, String),
    OpenOrder(OrderDataItem),
    AccountValue,
    PortfolioValue,
    AccountUpdateTime,
    NextValidId(i32),
    ContractData(i32, ContractDetails),
    ExecutionData,
    MarketDepth,
    MarketDepthL2,
    NewsBulletins,
    ManagedAccounts(String),
    ReceiveFa,
    HistoricalData(Bar),
    HistoricalDataEnd(i32, String, String),
    BondContractData,
    ScannerParameters,
    ScannerData(Vec<ScanDataItem>),
    TickOptionComputation,
    TickGeneric(i32, i32, f64),
    TickString,
    TickEfp,//TICK EFP 47
    CurrentTime,
    RealTimeBars(i32, Bar),
    FundamentalData,
    ContractDataEnd(i32),
    OpenOrderEnd,
    AccountDownloadEnd,
    ExecutionDataEnd,
    DeltaNeutralValidation,
    TickSnapshotEnd,
    MarketDataType,
    CommissionsReport,
    Position,
    PositionEnd,
    AccountSummary(AccountSummary),
    AccountSummaryEnd(i32),
    VerifyMessageApi,
    VerifyCompleted,
    DisplayGroupList,
    DisplayGroupUpdated,
    VerifyAndAuthMessageApi,
    VerifyAndAuthCompleted,
    PositionMulti,
    PositionMultiEnd,
    AccountUpdateMulti,
    AccountUpdateMultiEnd,
    SecurityDefinitionOptionParameter,
    SecurityDefinitionOptionParameterEnd,
    SoftDollarTier,
    FamilyCodes,
    SymbolSamples,
    MktDepthExchanges,
    TickReqParams,
    SmartComponents,
    NewsArticle,
    TickNews,
    NewsProviders,
    HistoricalNews,
    HistoricalNewsEnd,
    HeadTimestamp,
    HistogramData,
    HistoricalDataUpdate,
    RerouteMktDataReq,
    RerouteMktDepthReq,
    MarketRule,
    PnL(i32, f64, f64, f64),
    PnLsingle,
    HistoricalTick,
    HistoricalTickBidAsk,
    HistoricalTickLast,
    TickByTick,
    TickByTickLast((i32, TickLast)),
    OrderBound,
    CompletedOrder,
    CompletedOrdersEnd,
    Stop,
}
