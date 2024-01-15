use tokio;
use serde_json::{from_str, Value};
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};
use std::rc::Rc;
use crate::strategy::Strategy;
use crate::structure::*;
use tokio::sync::Mutex;


#[derive(Clone)]
pub struct Market<'a>
{
    pub streams: &'a str,
    pub base_ws: &'a str,
    pub stg: Rc<Mutex<Strategy<'a>>>,
}

impl<'a: 'static> Market<'a>
{
    pub fn new(streams: &'a str, stg: Rc<Mutex<Strategy<'a>>>) -> Self
    {
        Market {
            base_ws: "wss://fstream.binance.com/stream",
            streams: streams,
            stg: stg
        }
    }

    pub async fn start(&mut self)
    {
        // connect market streams and join the callback
        let client = self.ws_connect().await;
        let task = self.ws_callback(client);
        tokio::join!(task);
        // tokio::task::LocalSet::new().spawn_local(async move { task.await });
    }

    pub async fn ws_connect(&self) -> WebSocketStream<MaybeTlsStream<TcpStream>>
    {
        // make ws connection to market stream, return handshaked stream
        let url: String = self.base_ws.to_string() + &format!("?streams={}", self.streams).to_string();
        let client = connect_async(url)
        .await
        .unwrap();

        client.0
    }

    pub async fn ws_callback(&mut self, mut client: WebSocketStream<MaybeTlsStream<TcpStream>>)
    {
        // receive data from stream and handle it
        while let Some(msg) = client.next().await {
            let data = &(msg
                .unwrap()
                .into_text()
                .unwrap());
            
            // handle stream by content
            // to avoid borrow error, try and sleep and wait until the borrow is end
            if data.contains("aggTrade") {
                self.stg.lock().await.trade_callback(from_str::<AggTradeRtnWrap>(data).unwrap().data).await;
            }
            else if data.contains("bookTicker") {
                self.stg.lock().await.bookTicker_callback(from_str::<BookTickerRtnWrap>(data).unwrap().data).await;
            }
        }
    }
}


