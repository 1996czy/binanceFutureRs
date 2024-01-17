mod strategy;
mod market;
mod account;
mod rest;
mod structure;
use tokio;
use reqwest;
use std::rc::Rc;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    // initialize market and account module
    let mut stg = Rc::new(Mutex::new(strategy::Strategy::new().await));
    let mut mkt = market::Market::new();
    let mut act = account::Account::new();

    // start strategy
    stg.lock().await.start_handle_rsp().await;
    let task1 = mkt.start(stg.clone());
    let task2 = act.start(stg.clone());

    // make this event loop runforever
    let run_forever = async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100000)).await;
        }
    };
    tokio::join!(task1, task2, run_forever);
}
