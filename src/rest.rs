extern crate hex;
use reqwest;
use tokio;
use serde_json::{from_str, Value};
use serde_urlencoded::to_string;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use crate::structure::*;
type HmacSha256 = Hmac<Sha256>;


pub struct Rest {
}


impl Rest {
    pub fn new() -> Self {
        Rest {
        }
    }

    fn default_header() -> reqwest::header::HeaderMap {
        let pubkey = "*";
        let mut dh = reqwest::header::HeaderMap::new();
        dh.insert("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8;".parse().unwrap());
        dh.insert("X-MBX-APIKEY", pubkey.parse().unwrap());
        dh
    }

    fn method_client(method: &str, url: &str) -> Option<reqwest::RequestBuilder> {
        match method {
            "post" => Some(reqwest::Client::new().post(url)),
            "get" => Some(reqwest::Client::new().get(url)),
            "delete" => Some(reqwest::Client::new().delete(url)),
            "put" => Some(reqwest::Client::new().put(url)),
            &_ => None
        }
    }

    fn signature(query: &str) -> String {
        // given secret
        let secret = "*";
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("");
        mac.update(query.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    async fn make_request(header: reqwest::header::HeaderMap, sub_url: &str, body: Option<(&str, &str)>, method: &str) -> Result<reqwest::Response, reqwest::Error> {
        // give base rest
        let base_rest = "https://fapi.binance.com";
        // get corret x-www-form url
        let addr = match body {
            // sign request with body
            Some((&_, "sign")) => sub_url.to_string() + body.unwrap().0 + "&signature=" + &Self::signature(body.unwrap().0),
            // unsign request with body 
            Some((&_, &_)) => sub_url.to_string() + body.unwrap().0,
            // header only request
            None => sub_url.to_string()
        };
        // generate method client
        let client = Self::method_client(method, &(base_rest.to_string() + &addr));
        // make request
        client.unwrap()
        .headers(header)
        .send()
        .await
    }

    pub async fn get_exchangeInfo() -> ExchangeInfoRsp {
        // get address
        let addr = "/fapi/v1/exchangeInfo";
        // get listenkey req headers
        let header = Self::default_header();
        // make request
        let rsp = Self::make_request(header, addr, None, "get").await;
        // parse rsp
        rsp
        .unwrap()
        .json::<ExchangeInfoRsp>()
        .await
        .unwrap()
    }

    pub async fn get_listenKey() -> String {
        // listen key address
        let addr = "/fapi/v1/listenKey";
        // get listenkey req headers
        let header = Self::default_header();
        // make request
        let rsp = Self::make_request(header, addr, None, "post").await;
        // parse rsp
        rsp.expect("")
        .json::<ListenKey>()
        .await
        .expect("")
        .listenKey
    }

    pub async fn renew_listenKey(listen_key: String) {
        // listen key address
        let addr = "/fapi/v1/listenKey";
        // get listenkey req headers
        let header = Self::default_header();
        // make request
        let body = &to_string(ListenKey { listenKey: listen_key }).unwrap();
        let rsp = Self::make_request(header, addr, Some((body, "")), "put").await;
    }

    pub async fn create_order(&self, order: LimitOrder) {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = Self::default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = Self::make_request(header, addr, Some((body, "sign")), "post").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCreateRsp>()
        .await
        .expect("");
    }

    pub async fn amend_order(order: AmendOrder) {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = Self::default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = Self::make_request(header, addr, Some((body, "sign")), "put").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderAmendRsp>()
        .await
        .expect("");
    }

    pub async fn cancel_order(order: CancelOrder) -> String {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = Self::default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = Self::make_request(header, addr, Some((body, "sign")), "delete").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCancelRsp>()
        .await
        .expect("");
        "OK".to_string()
    }
    
    pub async fn cancel_all_order(order: CancelAllOrder) {
        // order address
        let addr = "/fapi/v1/allOpenOrders";
        // get order req headers
        let header = Self::default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = Self::make_request(header, addr, Some((body, "sign")), "delete").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCancelAllRsp>()
        .await
        .expect("");
    }
}
