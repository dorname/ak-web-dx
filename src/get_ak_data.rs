use reqwest::{self, Error};
use tokio::*;

pub async fn fetch_data(ip: &str, port: &str) -> Result<String, String> {
    let url = format!("http://{}:{}/api/public/stock_zh_a_hist", ip, port);
    
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
    let result = fetch_data("127.0.0.1", "8100").await;
    assert!(result.is_ok());
}
