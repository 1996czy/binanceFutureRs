use tokio;
use serde_json::Value;
use crate::rest::Rest;
use crate::structure::*;


pub struct Strategy<'a>
{
    pub rest: Rest<'a>,
}


impl<'a: 'static> Strategy<'a>
{
    pub fn new(rest: Rest<'a>) -> Self {
        Strategy {
            rest: rest
        }
    }

    pub async fn trade_callback(&mut self, data: AggTradeRtn) -> () {
        println!("{:?}", data.p);
    }

    pub async fn bookTicker_callback(&mut self, data: BookTickerRtn) -> () {
        println!("{:?}", data.A);
    }

    // update order by event type
    pub async fn order_update_callback(&mut self, order: &Value) {
        let instId: &str = order["s"].as_str().unwrap();
        let op: &str = order["x"].as_str().unwrap();
        match op {
            "NEW" => self.handle_new(order).await,
            "AMENDMENT" => self.handle_amend(order).await,
            "CANCELED" => self.handle_cancel(order).await,
            "TRADE" => self.handle_trade(order).await,
            &_ => println!("make no sense")
        }
    }
    
    async fn handle_new(&mut self, order: &Value) {
        
    }

    async fn handle_amend(&mut self, order: &Value) {

    }

    async fn handle_cancel(&mut self, order: &Value) {

    }

    async fn handle_trade(&self, order: &Value) {

    }

    async fn market_init_callback(&self) {
    
    }

    async fn trade_init_callback(&self) {
    
    }

}