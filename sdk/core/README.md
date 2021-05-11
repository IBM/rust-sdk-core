# IBM Cloud Core SDK 
[![Rust](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml/badge.svg)](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml)


![img.png](img.png)
#### This crate is an abstraction from de api referenced in docs.

The first abstraction is about the Identity Manager: https://cloud.ibm.com/apidocs/iam-identity-token-api.json

Single implementation of Authenticate with Token and API key:

Dependencies
```toml
#Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
ibmcloud-core = "0.1.2"
```
Main.rs
```rust
//src/main.rs
use ibmcloud_core::authenticators::token_api::{
    AuthenticatorApiClient, ResponseType, TokenResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You Atuhenticator Client
    // Need change your api-key
    let mut auth = AuthenticatorApiClient::new("<YOUR-API-key>".to_string());

    // This turn your client authenticated
    auth.authenticate().await?;
    println!("{:?}", auth.clone());
    
    // If you need to get token, this is the best choice. 
    // This feature manages the life cycle of your token.
    let token = auth.get_token();
    Ok(())
}

```
