mod strategy;
mod market;
mod account;
mod rest;
mod structure;
use tokio;
use reqwest;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    // let mut rsp = reqwest::get("https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=500").await.unwrap().json::<test>().await.unwrap();
    // println!("{:?}", rsp);
    // initialize market and account module
    let mut rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    let mut ls = tokio::task::LocalSet::new();
    let mut rest = rest::Rest::new();
    let mut stg = Rc::new(Mutex::new(strategy::Strategy::new(rest)));
    
    let streams = "btcusdt@aggTrade";
    let mut mkt = market::Market::new(streams, stg.clone());
    let mut act = account::Account::new(stg.clone());
    let task1 = mkt.start();
    let task2 = act.start();
    tokio::join!(task1, task2);


}
