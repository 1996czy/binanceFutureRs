use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use serde_json::from_str;
use futures::stream::StreamExt;
use webpki_roots::TLS_SERVER_ROOTS;
use rustls::{RootCertStore, ClientConfig, KeyLogFile};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async_tls_with_config, Connector};

use crate::strategy::Strategy;
use crate::structure::*;



pub struct Market {
}


impl Market {
    pub fn new() -> Self {
        Market {
        }
    }

    pub async fn start(&mut self, stg: Rc<Mutex<Strategy>>) {
        // connect market streams and join the callback
        let client = Self::ws_connect("btcusdt@aggTrade/ethusdt@aggTrade").await;
        let rsp = Self::ws_callback(client, stg).await;
    }

    async fn ws_connect(streams: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let base_ws = "wss://fstream.binance.com/stream";


        let mut root_cert_store = RootCertStore::empty();
        root_cert_store.extend(
            TLS_SERVER_ROOTS.iter().cloned()
        );

        let mut config = ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
        config.key_log = Arc::new(KeyLogFile::new());
        let connector = Connector::Rustls(Arc::new(config));

        // make ws connection to market stream, return handshaked stream
        let url: String = base_ws.to_string() + &format!("?streams={}", streams).to_string();
        connect_async_tls_with_config(url, None, false, Some(connector))
        .await
        .unwrap()
        .0
    }

    async fn ws_callback(mut client: WebSocketStream<MaybeTlsStream<TcpStream>>, stg: Rc<Mutex<Strategy>>) {
        // receive data from stream and handle it
        while let Some(msg) = client.next().await {
            let text_msg = &(msg
                .unwrap()
                .into_text()
                .unwrap());
            
            // handle stream by content
            if text_msg.contains("aggTrade") {
                stg
                .lock()
                .await
                .trade_callback(
                    from_str::<AggTradeRtnWrap>(text_msg).unwrap().data
                ).await;
            }
            else if text_msg.contains("bookTicker") {
                stg
                .lock()
                .await
                .bookTicker_callback(
                    from_str::<BookTickerRtnWrap>(text_msg).unwrap().data
                ).await;
            }
        }
    }
}


