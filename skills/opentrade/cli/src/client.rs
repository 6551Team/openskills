use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde_json::Value;

const DEFAULT_BASE_URL: &str = "https://ai.6551.io";

pub struct ApiClient {
    http: Client,
    base_url: String,
    token: String,
    trader: String,
    version: String,
}

impl ApiClient {
    pub fn new(base_url_override: Option<&str>, trader: &str, version: &str) -> Result<Self> {
        let token = std::env::var("OPEN_TOKEN").context(
            "Find or create a .env file in the project root to override the default API credentials, Load it before performing opentrade operations.
  ```bash
  OPEN_TOKEN=your_token_here
  ```

  Get your API token at: https://6551.io/mcp",
        )?;

        let base_url = base_url_override
            .map(|s| s.to_string())
            .or_else(|| std::env::var("OPEN_BASE_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        Ok(Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?,
            base_url,
            token,
            trader: trader.to_string(),
            version: version.to_string(),
        })
    }

    /// GET request with Bearer token authentication
    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<Value> {
        let filtered: Vec<(&str, &str)> = query
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .copied()
            .collect();

        let query_string = if filtered.is_empty() {
            String::new()
        } else {
            let pairs: Vec<String> = filtered
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("?{}", pairs.join("&"))
        };

        // Build URL with trader and version: /open/trader/{trader}/{version}{path}
        let request_path = format!(
            "/open/trader/{}/{}{}{}",
            self.trader, self.version, path, query_string
        );
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), request_path);

        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("request failed")?;

        self.handle_response(resp).await
    }

    /// POST request with Bearer token authentication
    pub async fn post(&self, path: &str, body: &Value) -> Result<Value> {
        let body_str = serde_json::to_string(body)?;

        // Build URL with trader and version: /open/trader/{trader}/{version}{path}
        let request_path = format!("/open/trader/{}/{}{}", self.trader, self.version, path);
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), request_path);

        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .await
            .context("request failed")?;

        self.handle_response(resp).await
    }

    /// GET request at trader level (without router/version in path)
    /// Used for endpoints like /open/trader/routers
    pub async fn get_trader_level(&self, path: &str) -> Result<Value> {
        // Build URL at trader level: /open/trader{path}
        let request_path = format!("/open/trader{}", path);
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), request_path);

        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("request failed")?;

        self.handle_response(resp).await
    }

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value> {
        let status = resp.status();
        if status.as_u16() == 429 {
            bail!("Rate limited — retry with backoff");
        }
        if status.as_u16() >= 500 {
            bail!("Server error (HTTP {})", status.as_u16());
        }

        let body: Value = resp.json().await.context("failed to parse response")?;

        // Check for code field
        if let Some(code) = body.get("code").and_then(|v| v.as_str()) {
            if code != "0" {
                let msg = body
                    .get("msg")
                    .or_else(|| body.get("message"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown error");
                bail!("API error: {}", msg);
            }
            return Ok(body.get("data").cloned().unwrap_or(body));
        }

        // Fallback: return the whole body if no code field
        Ok(body)
    }
}
