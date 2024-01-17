use serde::{Deserialize, Serialize};

// struct for listenKey
#[derive(Deserialize, Serialize, Debug)]
pub struct ListenKey {
    pub listenKey: String
}

// struct for create new order
#[derive(Serialize, Debug)]
pub struct LimitOrder {
    pub symbol: String,
    pub side: String,
    pub r#type: String,
    pub quantity: f32,
    pub price: f32,
    pub timeInForce: String,
    pub positionSide: String,
}

// struct for create new order
#[derive(Serialize, Debug)]
pub struct AmendOrder {
    pub symbol: String,
    pub orderId: i64,
    pub side: String,
    pub quantity: f32,
    pub price: f32
}

// struct for amend order
#[derive(Serialize, Debug)]
pub struct CancelOrder {
    pub orderId: i64,
    pub symbol: String
}

// struct for cancel all order
#[derive(Serialize, Debug)]
pub struct CancelAllOrder {
    pub symbol: String
}

// struct for order response from create order
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderCreateRsp {
    pub executedQty: String, 
    pub orderId: i64,
    pub avgPrice: String, 
    pub origQty: String, 
    pub price: String,
    pub side: String,
    pub positionSide: String
}

// struct for order response from amend order
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderAmendRsp {
    pub orderId: i64,
    pub price: String,
    pub avgPrice: String, 
    pub origQty: String, 
    pub executedQty: String, 
    pub side: String,
    pub positionSide: String
}

// struct for order response from cancel order
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderCancelRsp {
    pub orderId: i64
}

// struct for order response from cancel all order
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderCancelAllRsp {
    pub code: i64
}

// struct for market return of aggTrade
#[derive(Deserialize, Serialize, Debug)]
pub struct AggTradeRtn {
    pub e: String,          // Event type
    pub E: i64,             // Event time
    pub s: String,          // Symbol
    pub a: i64,             // Aggregate trade ID
    pub p: String,          // Price
    pub q: String,          // Quantity
    pub f: i64,             // First trade ID
    pub l: i64,             // Last trade ID
    pub T: i64,             // Trade time
    pub m: bool,            // Is the buyer the market maker?
}

// struct for market return of bookTicker
#[derive(Deserialize, Serialize, Debug)]
pub struct BookTickerRtn {
    pub e: String,          // Event type
    pub E: i64,             // Event time
    pub s: String,          // Symbol
    pub a: String,          // Aggregate trade ID
    pub b: String,          // Price
    pub A: String,          // Quantity
    pub B: String,          // First trade ID
    pub T: i64,             // Trade time
}


#[derive(Deserialize, Serialize, Debug)]
pub struct OrderUpdateRtn {
    pub s: String,      // Symbol
    pub S: String,      // Side
    pub q: String,      // Original Quantity
    pub p: String,      // Original Price
    pub x: String,      // Execution Type
    pub X: String,      // Order Status
    pub i: i64,         // Order Id
    pub l: String,      // Order Last Filled Quantity
    pub T: i64,         // Order Trade Time
    pub t: i64,         // Trade Id
    pub ps: String,     // Position Side
    pub rp: String,     // Realized Profit of the trade
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AggTradeRtnWrap {
    pub stream: String,            // stream name
    pub data: AggTradeRtn,         // data stream
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookTickerRtnWrap {
    pub stream: String,            // stream name
    pub data: BookTickerRtn,         // data stream
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderUpdateRtnWrap {
    pub e: String,          // Event Type
    pub E: i64,             // Event Time
    pub T: i64,             // Transaction Time
    pub o: OrderUpdateRtn
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RateLimits {
    pub interval: String,
    pub intervalNum: i32,
    pub limit: i32,
    pub rateLimitType: String 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TradeParam {
    pub filterType: String,
    pub stepSize: Option<String>, 
    pub tickSize: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExchangeSymbols {
    pub symbol: String,
    pub filters: Vec<TradeParam>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExchangeInfoRsp {
    pub rateLimits: Vec<RateLimits>,
    pub symbols: Vec<ExchangeSymbols>
}

