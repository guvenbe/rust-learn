use serde::{Deserialize, Serialize};

#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "networking")]
use reqwest;

#[cfg(feature = "logging")]
use tracing;

#[derive(Serialize, Deserialize, Debug)]  // Debug always available
pub struct ApiResponse {
    pub status: u32,
    pub message: String,
    
    #[cfg(feature = "tls")]
    pub encrypted: bool,
}

pub struct HttpClient {
    #[cfg(feature = "networking")]
    client: reqwest::Client,
    
    #[cfg(feature = "logging")]
    logger: tracing::Span,
}

impl HttpClient {
    #[cfg(feature = "networking")]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "logging")]
        let logger = tracing::info_span!("http_client");
        
        Ok(HttpClient {
            client: reqwest::Client::new(),
            #[cfg(feature = "logging")]
            logger,
        })
    }
    
    #[cfg(not(feature = "networking"))]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Err("Networking feature is required for HTTP client functionality".into())
    }
    
    #[cfg(all(feature = "networking", feature = "json"))]
    pub async fn fetch_json<T>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        #[cfg(feature = "logging")]
        tracing::info!("Fetching JSON from: {}", url);
        
        let response = self.client.get(url).send().await?;
        let json_data = response.json::<T>().await?;
        
        Ok(json_data)
    }
    
    #[cfg(feature = "tls")]
    pub fn is_secure(&self) -> bool {
        true
    }
    
    #[cfg(not(feature = "tls"))]
    pub fn is_secure(&self) -> bool {
        false
    }
}

// Platform-specific modules using built-in cfg attributes
#[cfg(unix)]
pub mod unix_specific {
    pub fn platform_info() -> &'static str {
        "Running on Unix-like system"
    }
}

#[cfg(windows)]
pub mod windows_specific {
    pub fn platform_info() -> &'static str {
        "Running on Windows system"
    }
}

// Example usage function
pub fn demonstrate_features() {
    #[cfg(feature = "logging")]
    tracing::info!("Starting feature demonstration");
    
    #[cfg(feature = "json")]
    {
        let data = r#"{"status": 200, "message": "success"}"#;
        let _parsed: ApiResponse = serde_json::from_str(data).unwrap();
        println!("JSON parsing feature is enabled");
    }
    
    #[cfg(feature = "tls")]
    println!("TLS support is enabled");
    
    #[cfg(feature = "networking")]
    println!("Networking capabilities are available");
}