use ibmcloud_core::authenticators::token_api::{
    AuthenticatorApiClient, ResponseType, TokenResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth = AuthenticatorApiClient::new("<YOUR-API-key>".to_string());
    auth.authenticate().await?;
    println!("{:?}", auth.clone());

    let token = auth.get_token().await;
    Ok(())

}
