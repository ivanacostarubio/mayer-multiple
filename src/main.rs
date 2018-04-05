#![feature(rustc_private)]
extern crate hyper;
extern crate chrono;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate serialize;
extern crate rustc_serialize;



use serde_json::Value;


use std::io::Read;
use hyper::Client;

use chrono::prelude::*;
use time::Duration;




fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let moving_average = get_two_hundred_days_average();
    let last_price = get_latest_price();
    let multiplier = last_price / moving_average;

    println!("{0:<15} - {1:?}", "Moving Average", moving_average);
    println!("{0:<15} - {1:?}", "Last Price", last_price);
    println!("{0:<15} - {1:?}", "Multiple" ,multiplier);
}

fn get_latest_price() -> f64{
    let url = "https://api.coindesk.com/v1/bpi/currentprice.json";
    let content= get_content(&url);
    let as_str = content.unwrap();
    let value: Value= serde_json::from_str(&as_str).unwrap();
    let latest = value.get("bpi").unwrap().get("USD").unwrap().get("rate_float").unwrap();
    let result = latest.as_f64().unwrap();
    
    return result;
}

fn get_two_hundred_days_average() -> f64 {

    let today: DateTime<Utc> = Utc::now();
    let two_hundred_days_ago = today - Duration::days(200);

    let f_today = format!("{}-{:02}-{:02}", today.year(), today.month(), today.day());
    let f_twohundred_days_ago = format!("{}-{:02}-{:02}", two_hundred_days_ago.year(), two_hundred_days_ago.month(), two_hundred_days_ago.day());



    let url = format!("{}{}{}{}", "https://api.coindesk.com/v1/bpi/historical/close.json?start=", f_twohundred_days_ago, "&end=" , f_today).to_string();

    let content = get_content(&url);
    let as_str = content.unwrap();
    let value: Value= serde_json::from_str(&as_str).unwrap();
    let mut precios = Vec::new();

    for x in value.get("bpi") {
        let aver = vec![x];
        for (_k,v) in aver[0].as_object().unwrap(){
            precios.push(v.as_f64().unwrap());
        }
    }

    let total_price: f64 = precios.iter().sum();
    return total_price / 200.0;

}
