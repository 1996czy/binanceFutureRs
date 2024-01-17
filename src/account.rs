use std::rc::Rc;
use std::sync::Arc;
use serde_json::from_str;
use tokio::sync::Mutex;
use tokio::{time, spawn};
use tokio::net::TcpStream;
use futures::stream::StreamExt;
use webpki_roots::TLS_SERVER_ROOTS;
use rustls::{RootCertStore, ClientConfig, KeyLogFile};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async_tls_with_config, Connector};

use crate::strategy::Strategy;
use crate::rest::Rest;
use crate::structure::*;


pub struct Account {
}


impl Account {
    pub fn new() -> Self {
        Account {
        }
    }

    pub async fn start(&mut self, stg: Rc<Mutex<Strategy>>) {
        // use listenKey to connect user streams and join the callback
        let listen_key = Rest::get_listenKey().await;

        // spawn a thread to update listen key
        let listen_key_copy: String = listen_key.clone();
        spawn(async move { loop {
            let tmp = listen_key_copy.clone();
            Rest::renew_listenKey(tmp).await;
            time::sleep(time::Duration::from_millis(1000000)).await;
        }});

        // connect Account Stream
        let client = Self::ws_connect(&listen_key).await;
        let rsp = Self::ws_callback(client, stg).await;
    }

    async fn ws_connect(listen_key: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let base_ws = "wss://fstream.binance.com/ws";

        let mut root_cert_store = RootCertStore::empty();
        root_cert_store.extend(
            TLS_SERVER_ROOTS.iter().cloned()
        );

        let mut config = ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
        config.key_log = Arc::new(KeyLogFile::new());
        let connector = Connector::Rustls(Arc::new(config));

        // make ws connection to user stream, return handshaked stream
        let url: String = base_ws.to_string() + &format!("/{}", listen_key).to_string();
        connect_async_tls_with_config(url, None, false, Some(connector))
        .await
        .unwrap()
        .0
    }

    async fn ws_callback(mut client: WebSocketStream<MaybeTlsStream<TcpStream>>, stg: Rc<Mutex<Strategy>>) {
        // receive data from stream and handle it
        while let Some(msg) = client.next().await {
            
            let text_msg = msg
                .unwrap()
                .into_text()
                .unwrap();

            match text_msg.contains("ORDER_TRADE_UPDATE") {

                true => {
                    // handle data by content
                    let data = from_str::<OrderUpdateRtnWrap>(&text_msg).unwrap();
                    // callback
                    stg.lock()
                    .await
                    .order_update_callback(data.o)
                    .await;
                },

                false => println!("currently unused data")
            }
        }
    }
}
