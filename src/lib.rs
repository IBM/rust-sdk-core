pub mod authenticators;


#[cfg(test)]
mod TokenApiTests{
    const ibm_cloud_iam_url: &str = "ibm_cloud_iam_url";
    const api_key:  &str= "api_key";
    const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

    use crate::authenticators::token_api::{AuthenticatorApiClient, TokenApiKeyRequest};

    #[test]
    fn new_authenticator_client_success(){
        let auth = AuthenticatorApiClient::new(ibm_cloud_iam_url.to_string());

        assert_eq!(auth, AuthenticatorApiClient{ url: ibm_cloud_iam_url.to_string() })
    }
    fn new_token_api_request_success(){
        let req = TokenApiKeyRequest::new(api_key.to_string());

        assert_eq!(req, TokenApiKeyRequest{ grant_type: GRANT_TYPE.to_string(), apikey: api_key.to_string() })
    }



}