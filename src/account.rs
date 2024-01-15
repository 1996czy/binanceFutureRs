use reqwest;
use tokio;
use serde_json::{from_str, Value};
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};
use std::rc::Rc;
use crate::strategy::Strategy;
use crate::rest::Rest;
use tokio::sync::Mutex;


#[derive(Clone)]
pub struct Account<'a>
{
    base_ws: &'a str,
    base_rest: &'a str,
    stg: Rc<Mutex<Strategy<'a>>>,
}

impl<'a: 'static> Account<'a>
{
    pub fn new(stg: Rc<Mutex<Strategy<'a>>>) -> Self {
        Account {
            base_ws: "wss://fstream.binance.com/ws",
            base_rest: "https://fapi.binance.com",
            stg: stg
        }
    }

    pub async fn start(&mut self)
    {
        // use listenKey to connect user streams and join the callback
        let listen_key = self.stg.lock().await.rest.get_listenKey().await;
        let client = self.ws_connect(&listen_key).await;
        let rsp = self.stg.lock().await.rest.renew_listenKey(&listen_key).await;
        let task = self.ws_callback(client);
        tokio::join!(task);
    }

    async fn ws_connect(&self, listen_key: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>>
    {
        // make ws connection to user stream, return handshaked stream
        let url: String = self.base_ws.to_string() + &format!("/{}", listen_key).to_string();
        let client = connect_async(url).await.unwrap();
        client.0
    }

    async fn ws_callback(&mut self, mut client: WebSocketStream<MaybeTlsStream<TcpStream>>)
    {
        // receive data from stream and handle it
        while let Some(msg) = client.next().await {
            let data = from_str::<Value>(&(msg
                .unwrap()
                .into_text()
                .unwrap())).unwrap();

            
            // handle stream by content
            match data["e"].as_str()
            {
                Some("ORDER_TRADE_UPDATE") => {
                    self.stg.lock().await.order_update_callback(&data["o"]).await;
                }

                Some(&_) => println!("currently unused data"),

                None => println!("None data"),
            }
        }
    }
}