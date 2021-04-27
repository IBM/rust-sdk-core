# IBM Cloud Core SDK 
[![Rust](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml/badge.svg)](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml)

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
use ibmcloud_core::authenticators::token_api::{AuthenticatorApiClient, TokenApiKeyRequest, ResponseType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //The client for configure the conection with IBM cloud
    let auth = AuthenticatorApiClient::new("https://iam.cloud.ibm.com/identity/token".to_string());
    
    //Configuration for request token with an API key
    let req = TokenApiKeyRequest::new("<YOUR-API-key>".to_string());
    
    //The async function for get authentication with the api key
    let result = auth.authenticate(req).await?;
    
    // Return is a representation of the Response by api.
    match result{
        ResponseType::Ok(token)=>{
            //Get the token in the reponse
            println!("{:?}",token.get_access_token());
        }
        ResponseType::Err(e)=>{
            println!("{:?}",e);
        }
    }

    Ok(())
}

```
