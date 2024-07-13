use reqwest; // 确保你的依赖项中有 reqwest 包。
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::*;

const BASE_URL:&str = "http://127.0.0.1:8100/api/public";
pub async fn fetch_data(api_name: &str, pm: HashMap<String, String>) -> Result<String, String> {
    let base_url = format!("{}/{}", BASE_URL, api_name);
    let client = reqwest::Client::new();

    // 构建带参数的 URL。
    let url = client
        .get(&base_url)
        .query(&pm)
        .build().unwrap()
        .url()
        .to_string();

    println!("Request URL: {}", url);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => Ok(body),
                    Err(_) => Err("Failed to read body".to_string())
                }
            } else {
                Err(format!("Request failed with status: {}", response.status()))
            }
        },
        Err(e) => Err(format!("Failed to make request: {}", e))
    }
}


#[tokio::test]
async fn test_fetch_data() {
    use crate::model;
    let mut pm = HashMap::<String,String>::new();
    pm.insert("symbol".to_owned(),"600076".to_owned());
    pm.insert("start_date".to_owned(),"20240711".to_owned());
    let result = fetch_data("stock_zh_a_hist", pm).await;
    assert!(result.is_ok());
}
