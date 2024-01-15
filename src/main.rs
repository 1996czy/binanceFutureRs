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

#[derive(Deserialize, Serialize, Debug)]
struct test {
    lastUpdateId: i64,
    E: i64,
    T: i64,
}


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


    // let mut stg = Arc::new(Mutex::new(strategy::Strategy::new(rest)));
    // ls.block_on(&mut rt, async {
    //     let handle1 = tokio::task::spawn_local(async {
    //         mkt.start().await
    //     });
    //     let handle2 = tokio::task::spawn_local(async {
    //         act.start().await
    //     });
    //     tokio::join!(handle1, handle2);
    // });
    // tokio::join!(task1, task2);
    // set.spawn_local(move || async {task1.await});
    // set.spawn_blocking(async move || {task2});

    // while let Some(res) = set.join_next().await {
    //     let idx = res.unwrap();
    // }

    // ls.run_until(mkt.start()).await;
    // tokio::join!(task1);
    // tokio::join!(ls);
    // drop(ls);
    // tokio::join!(ls);
    // rt.spawn_local(task1);
    // rt.spawn_local(task2);

    // make this event loop runforever
    let run_forever = async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100000)).await;
        }
    };
    tokio::join!(run_forever);
}

// struct Sleeper {
//     text: i32
// }

// impl Sleeper {
//     pub async fn sleep_task(&mut self) {
//         tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
//         self.text += 1;
//         println!("{:?}", self.text);
//     }
// }


// struct CallSleeper<F>
// where F: Future<Output = ()>
// {
//     sleeper: Rc<RefCell<F>>
// }


// impl<F> CallSleeper<F>
// where F: Future<Output = ()>
// {
//     async fn call(&mut self) {
//         (self.sleeper.borrow_mut()).await;
//     }
// }

// #[tokio::main]
// async fn main() {
    
//     // let mut sp = Sleeper { text: 1 };
//     // let mut s1 = CallSleeper { sleeper: Rc::new(RefCell::new(sp.sleep_task())) };
//     // let mut s2 = CallSleeper { sleeper: Rc::new(RefCell::new(sp.sleep_task())) };
//     // let task1 = s1.call();
//     // let task2 = s2.call();
//     // tokio::join!(task1, task2);
// }