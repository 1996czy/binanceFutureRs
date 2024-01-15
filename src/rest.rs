extern crate hex;
use reqwest;
use tokio;
use serde_json::{from_str, Value};
use serde_urlencoded::to_string;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use crate::structure::*;
type HmacSha256 = Hmac<Sha256>;


pub struct Rest<'a> {
    base_rest: &'a str,
    pubkey: &'a str,
    secret: &'a str,
}


impl<'a> Rest<'a> {
    pub fn new() -> Self {
        Rest {
            base_rest: "https://fapi.binance.com",
            pubkey: "***",
            secret: "***"
        }
    }

    fn default_header(&self) -> reqwest::header::HeaderMap {
        let mut dh = reqwest::header::HeaderMap::new();
        dh.insert("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8;".parse().unwrap());
        dh.insert("X-MBX-APIKEY", self.pubkey.parse().unwrap());
        dh
    }

    fn method_client(&self, method: &str, url: &str) -> Option<reqwest::RequestBuilder> {
        match method {
            "post" => Some(reqwest::Client::new().post(url)),
            "get" => Some(reqwest::Client::new().get(url)),
            "delete" => Some(reqwest::Client::new().delete(url)),
            "put" => Some(reqwest::Client::new().put(url)),
            &_ => None
        }
    }

    fn signature(&self, query: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).expect("");
        mac.update(query.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    async fn make_request(&self, header: reqwest::header::HeaderMap, sub_url: &str, body: Option<(&str, &str)>, method: &str) -> Result<reqwest::Response, reqwest::Error> {
        // get corret x-www-form url
        let mut addr = match body {
            // sign request with body
            Some((&_, "sign")) => sub_url.to_string() + body.unwrap().0 + "&signature=" + &self.signature(body.unwrap().0),
            // unsign request with body 
            Some((&_, &_)) => sub_url.to_string() + body.unwrap().0,
            // header only request
            None => sub_url.to_string()
        };
        // generate method client
        let client = self.method_client(method, &(self.base_rest.to_string() + &addr));
        // make request
        client.unwrap()
        .headers(header)
        .send()
        .await
    }

    pub async fn get_listenKey(&self) -> String{
        // listen key address
        let addr = "/fapi/v1/listenKey";
        // get listenkey req headers
        let header = self.default_header();
        // make request
        let rsp = self.make_request(header, addr, None, "post").await;
        // parse rsp
        rsp.expect("")
        .json::<ListenKey>()
        .await
        .expect("")
        .listenKey
    }

    pub async fn renew_listenKey(&self, listen_key: &str) {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        // listen key address
        let addr = "/fapi/v1/listenKey";
        // get listenkey req headers
        let header = self.default_header();
        // make request
        let body = &to_string(ListenKey { listenKey: listen_key.to_string() }).unwrap();
        let rsp = self.make_request(header, addr, Some((body, "")), "put").await;
    }

    pub async fn create_order(&self, order: LimitOrder) {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = self.default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = self.make_request(header, addr, Some((body, "sign")), "post").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCreateResponse>()
        .await
        .expect("");
    }

    pub async fn amend_order(&self, order: AmendOrder) {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = self.default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = self.make_request(header, addr, Some((body, "sign")), "put").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderAmendResponse>()
        .await
        .expect("");
    }

    pub async fn cancel_order(&self, order: CancelOrder) {
        // order address
        let addr = "/fapi/v1/order";
        // get order req headers
        let header = self.default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = self.make_request(header, addr, Some((body, "sign")), "delete").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCancelResponse>()
        .await
        .expect("");
    }
    
    pub async fn cancel_all_order(&self, order: CancelAllOrder) {
        // order address
        let addr = "/fapi/v1/allOpenOrders";
        // get order req headers
        let header = self.default_header();
        // make request
        let body = &to_string(&order).unwrap();
        let rsp = self.make_request(header, addr, Some((body, "sign")), "delete").await;
        // parse rsp
        rsp.expect("")
        .json::<OrderCancelAllResponse>()
        .await
        .expect("");
    }
}