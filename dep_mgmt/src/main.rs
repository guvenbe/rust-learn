// main.rs
use my_http_client::{HttpClient, ApiResponse};

#[tokio::main]
async fn main() {
    // This will only compile if networking feature is enabled
    #[cfg(feature = "networking")]
    {
        match HttpClient::new() {
            Ok(client) => {
                println!("Secure connection: {}", client.is_secure());
                
                // JSON functionality only available with both networking and json features
                #[cfg(feature = "json")]
                {
                    // Example API call (would need a real URL)
                    // let api_response: ApiResponse = client.fetch_json("https://api.example.com/data").await.unwrap();
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    // Always available
    my_http_client::demonstrate_features();
    
    // Platform-specific code
    #[cfg(unix)]
    println!("{}", my_http_client::unix_specific::platform_info());
    
    #[cfg(windows)]
    println!("{}", my_http_client::windows_specific::platform_info());
}