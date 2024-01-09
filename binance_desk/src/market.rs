use tokio;
use serde_json::{from_str, Value};
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};


pub struct Market<'a>
{
    streams: &'a str,
    base_ws: &'a str
}

impl<'a> Market<'a>
{
    pub fn new(streams: &'a str) -> Self {
        Market {
            base_ws: "wss://fstream.binance.com/stream",
            streams: streams,
        }
    }

    pub async fn start(&'a mut self)
    {
        // connect market streams and join the callback
        let client = self.ws_connect().await;
        let task = self.ws_callback(client);
        tokio::join!(task);
    }

    async fn ws_connect(&self) -> WebSocketStream<MaybeTlsStream<TcpStream>>
    {
        // make ws connection to market stream, return handshaked stream
        let url: String = self.base_ws.to_string() + &format!("?streams={}", self.streams).to_string();
        let client = connect_async(url)
        .await
        .unwrap();

        client.0
    }

    async fn ws_callback(&self, mut client: WebSocketStream<MaybeTlsStream<TcpStream>>)
    {
        // receive data from stream and handle it
        while let Some(msg) = client.next().await {
            let data = from_str::<Value>(&(msg
                .unwrap()
                .into_text()
                .unwrap()))
                .unwrap();

            let stream = data["stream"]
            .as_str()
            .unwrap();

            // handle stream by content
            if stream.contains("aggTrade") {
                Self::trade_callback(data.get("data").unwrap()).await;
            }
            else if stream.contains("bookTicker"){
                Self::bookTicker_callback(data.get("data").unwrap()).await;
            }

        }
    }
    
    async fn trade_callback(data: &Value)
    {
        println!("{:?}", data);
    }

    async fn bookTicker_callback(data: &Value)
    {
        println!("{:?}", data)
    }

}


