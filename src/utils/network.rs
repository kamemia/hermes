use reqwest::{Client, Method};
use serde_json::json;

use crate::request::{RequestState, ResponseState};

pub async fn send_request(request: RequestState) -> Result<ResponseState, String> {
    let client = Client::new();

    let mut payload = client
        .request(
            Method::from_bytes(request.method.as_bytes()).unwrap(),
            request.url,
        )
        .body(request.body);

    for (key, value) in request.headers {
        payload = payload.header(key, value);
    }

    let response = payload.send().await;

    match response {
        Ok(response) => {
            let response_state = ResponseState {
                status_code: response.status().as_u16(),
                headers: response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                    .collect(),
                body: response.text().await.unwrap(),
            };
            return Ok(response_state);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}

pub fn format_json(json: String) -> String {
    let json_value: serde_json::Value = serde_json::from_str(&json).unwrap_or_else(|error| {
        json!({
            "hermes_error": error.to_string(),
            "message": "Failed to parse JSON value",
            "text": json
        })
    });

    // format JSON
    let formatted = serde_json::to_string_pretty(&json_value).unwrap_or_else(|error| {
        format!(
            "Failed to format JSON\nError: {}\nText: {}",
            error.to_string(),
            json
        )
    });

    return formatted;
}
