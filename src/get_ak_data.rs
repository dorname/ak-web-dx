use reqwest::{self};
use crate::params::*;
use tokio::*;
pub async fn fetch_data(ip: &str, port: &str) -> Result<String, String> {
    let url = format!("http://{}:{}/api/public/stock_zh_a_hist?symbol={}&start_date={}&end_date={}", ip, port,"600076","20240710","20240712");
    // let url = format!("http://{}:{}/api/public/stock_zh_a_hist?symbol={}", ip, port,"600076");
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => {
                        println!("{}", body);
                        Ok(body)
                    },
                    Err(_) => Err(String::from("Failed to read body"))
                }
            } else {
                Err(String::from("Request failed"))
            }
        },
        Err(_) => Err(String::from("Failed to make request"))
    }
}

#[tokio::test]
async fn test_fetch_data() {
    use crate::model;
    let result = fetch_data("127.0.0.1", "8100").await;
    assert!(result.is_ok());
    let stocks: Vec<model::stock_zh_model::ZnStocks> = serde_json::from_str(result.unwrap().as_str()).expect("JSON was not well-formatted");
    for stock in stocks{
        println!("{:?}",stock);
    }
}
