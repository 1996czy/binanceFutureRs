pub mod market;
pub mod account;
use tokio;


#[tokio::main]
async fn main()
{
    // initialize market and account module
    let mut m = market::Market::new("footballusdt@aggTrade");
    let mut a = account::Account::new();
    let task1 = m.start();
    let task2 = a.start();

    // make this event loop runforever
    let run_forever = async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100000)).await;
        }
    };
    tokio::join!(task1, task2, run_forever);
}
