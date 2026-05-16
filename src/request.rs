pub const METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

#[derive(Debug, Clone)]
pub struct RequestState {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Default for RequestState {
    fn default() -> Self {
        Self {
            url: String::from("https://rickandmortyapi.com/api"),
            method: String::from("GET"),
            headers: vec![
                // JSON by default
                (
                    String::from("Content-Type"),
                    String::from("application/json"),
                ),
            ],
            body: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseState {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
