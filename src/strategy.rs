use tokio;
use async_scoped::TokioScope;
use std::collections::HashMap;
use crate::rest::Rest;
use crate::structure::*;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::sync::Mutex;
use std::sync::Arc;
type Am<T> = Arc<Mutex<T>>;
type Amh<A, T> = Arc<Mutex<HashMap<A, T>>>;


pub struct Strategy {
    
}


impl Strategy {
    pub async fn new() -> Self {

        // get exchange info
        let exchange_info = futures::executor::block_on(Rest::get_exchangeInfo());

        // get rate limits
        let min_weight = exchange_info.rateLimits[0].limit;
        let min_orders = exchange_info.rateLimits[1].limit;
        let sec_orders = exchange_info.rateLimits[2].limit;

        // get symbol to trade
        for symbol in &exchange_info.symbols[0..5] {
            // get instrument id
            let s: String = (&symbol.symbol).to_string();
            // get size precision of contract
            let step_size = symbol
                .filters[1]
                .stepSize
                .clone()
                .unwrap()
                .parse::<f64>()
                .unwrap();
            // get price precision of contract
            let tick_size = (1.0 / 
                (symbol
                    .filters[0]
                    .tickSize
                    .clone()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()))
                .ceil()
                .to_string()
                .len() - 1;

        }

        let (tx, mut rx) = channel(100);
        let sender = Arc::new(Mutex::new(tx));
        let receiver = Arc::new(Mutex::new(rx));

        Strategy {
            
        }
    }

    pub async fn start_handle_rsp(&mut self) {
        tokio::spawn(Self::handle_rsp(self.receiver.clone(), self.long_transit.clone(), self.short_transit.clone()));
    }

    async fn handle_rsp(receiver: Am<Receiver<String>>, long_transit: Amh<String, bool>, short_transit: Amh<String, bool>) {
        while let Some(rsp) = receiver.lock().await.recv().await {
        }
    }

    async fn test(inst_id: String, sender: Am<Sender<String>>) {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        sender.lock().await.send(inst_id).await;
    }

    async fn create_long(&mut self, inst_id: &str) {
        tokio::spawn(Self::test(inst_id.to_string(), self.sender.clone()));
    }

    async fn create_short(&mut self, inst_id: &str) {
        tokio::spawn(Self::test(inst_id.to_string(), self.sender.clone()));
    }

    async fn update_order(&mut self, mid_px: f64, inst_id: &str) {
        
    }

    pub async fn trade_callback(&mut self, data: AggTradeRtn) {
        let inst_id: &str = &data.s;
        // when trading and no position currently, update maker order
        if self.trading && *self.long_pos.get(inst_id).unwrap() + *self.short_pos.get(inst_id).unwrap() == 0 {
            self.update_order(data.p.parse::<f64>().unwrap(), inst_id).await;
        }
    }

    pub async fn bookTicker_callback(&mut self, data: BookTickerRtn) {
        println!("{:?}", data.A);
    }

    // update order by event type
    pub async fn order_update_callback(&mut self, order: OrderUpdateRtn) {
        let instId: &str = &order.s;
        let op: &str = &order.x;
        match op {
            "NEW" => self.handle_new(order).await,
            "AMENDMENT" => self.handle_amend(order).await,
            "CANCELED" => self.handle_cancel(order).await,
            "TRADE" => self.handle_trade(order).await,
            &_ => println!("make no sense")
        }
    }
    
    async fn handle_new(&mut self, order: OrderUpdateRtn) {
        
    }

    async fn handle_amend(&mut self, order: OrderUpdateRtn) {

    }

    async fn handle_cancel(&mut self, order: OrderUpdateRtn) {

    }

    async fn handle_trade(&self, order: OrderUpdateRtn) {

    }

    async fn market_init_callback(&self) {
    
    }

    async fn trade_init_callback(&self) {
    
    }
}
