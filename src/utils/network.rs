use reqwest::{Client, Method};
use serde_json;

pub async fn send_request(url: String, method: String) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .request(Method::from_bytes(method.as_bytes()).unwrap(), url)
        .send()
        .await;

    match response {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&text).unwrap();
            let formatted = serde_json::to_string_pretty(&json_value).unwrap();

            return Ok(formatted);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}
