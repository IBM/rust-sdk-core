# IBM Cloud Core SDK 
[![Rust](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml/badge.svg)](https://github.com/rogeriob2br/ibmcloud-core/actions/workflows/rust.yml)

#### This crate is an abstraction from de api referenced in docs.

The first abstraction is about the Identity Manager: https://cloud.ibm.com/apidocs/iam-identity-token-api.json

Single implementation of Authenticate with Token and API key:

```rust
use crate::assistant::v2::AssistantClient;
use crate::authenticators::token_api::{AuthenticatorApiClient, TokenApiKeyRequest};

#[tokio::main]
async fn main() {


    let auth = AuthenticatorApiClient::new("https://iam.cloud.ibm.com/identity/token".to_string());
    let req = TokenApiKeyRequest::new("<YOUR-API-KEY>".to_string());
    let token = auth.authenticate(req).await;
    println!("{:?}",token);

}
```
