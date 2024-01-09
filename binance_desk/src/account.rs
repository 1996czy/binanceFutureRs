use reqwest;
use tokio;
use serde::Deserialize;
use serde_json::{from_str, Value};
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};


#[derive(Deserialize, Debug)]
struct LK {
    listenKey: String
}

pub struct Account<'a>
{
    base_ws: &'a str,
    base_rest: &'a str,
    listen_addr: &'a str
}

impl<'a> Account<'a>
{
    pub fn new() -> Self {
        Account {
            base_ws: "wss://fstream.binance.com/ws",
            base_rest: "https://fapi.binance.com",
            listen_addr: "/fapi/v1/listenKey"
        }
    }

    pub async fn start(&'a mut self)
    {
        // get listenkey req headers
        let mut listen_header = reqwest::header::HeaderMap::new();
        listen_header.insert("Content-Type", "application/json".parse().unwrap());
        listen_header.insert("X-MBX-APIKEY", "key".parse().unwrap());
        
        // make post
        let listen_url = self.base_rest.to_string() + self.listen_addr;
        let client = reqwest::Client::new();
        let rsp = client.post(listen_url)
        .headers(listen_header)
        .send()
        .await;
        
        // parse rsp
        let rsp =  rsp.expect("")
        .json::<LK>()
        .await;
        let listen_key = rsp
        .expect("")
        .listenKey;
        
        // use listenKey to connect user streams and join the callback
        let client = self.ws_connect(&listen_key).await;
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

    async fn ws_callback(&self, mut client: WebSocketStream<MaybeTlsStream<TcpStream>>)
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
                Some("ORDER_TRADE_UPDATE") => Self::order_update_callback(&data["o"]).await,

                Some(&_) => println!("currently unused data"),

                None => println!("None data"),
            }
        }
    }

    async fn order_update_callback(order: &Value)
    {
        println!("{:?}", order);
    }
}