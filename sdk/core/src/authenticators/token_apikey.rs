use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use std::env;
use crate::authenticators::base::{ResponseType, TokenResponse, Configs};

const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

#[derive(Deserialize, Debug, Clone)]
pub struct AuthenticatorApiClient {
    pub(crate) url: String,
    pub(crate) token: TokenResponse,
    pub(crate) options: Options,
}

impl AuthenticatorApiClient {
    pub fn new(apikey: String) -> AuthenticatorApiClient {
        let config = Configs::new();
        AuthenticatorApiClient {
            url: config.IAM_IDENTITY_URL,
            token: TokenResponse {
                access_token: "".to_string(),
                refresh_token: None,
                delegated_refresh_token: None,
                token_type: "".to_string(),
                expires_in: 0,
                expiration: 0,
            },
            options: Options::new(apikey),
        }
    }
    fn set_token(&mut self, token: TokenResponse) {
        self.token = token
    }

    pub async fn get_token(&mut self) -> TokenResponse {
        if self.token.validate_token() {
            self.token.clone()
        } else {
            self.authenticate().await;
            self.token.clone()
        }
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        let response = request_token(self.options.clone(), String::from(&self.url.clone())).await?;

        match response.clone() {
            ResponseType::Ok(TokenResponse) => {
                self.set_token(TokenResponse);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

async fn request_token(req: Options, url: String) -> Result<ResponseType> {
    let params = urlencoded_parameter(req);
    let response: ResponseType = reqwest::Client::new()
        .post(url)
        .form(&params)
        .headers(construct_headers())
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Options {
    //Grant type for this API call. You must set the
    // grant type to urn:ibm:params:oauth:grant-type:apikey.
    pub(crate) grant_type: String,

    //The value of the api key.
    pub(crate) apikey: String,
}

impl Options {
    pub fn new(apikey: String) -> Options {
        Options {
            grant_type: GRANT_TYPE.to_string(),
            apikey,
        }
    }
}



/// Helpers------------------------------------------------

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}

fn urlencoded_parameter(token: Options) -> [(String, String); 2] {
    let params: [(String, String); 2] = [
        ("grant_type".to_string(), token.grant_type),
        ("apikey".to_string(), token.apikey),
    ];
    params
}

#[cfg(test)]
mod TokenApiTests {
    use crate::authenticators::token_apikey::{
        construct_headers,  urlencoded_parameter, Options
    };
    use chrono::Local;

    use reqwest::header::{ CONTENT_TYPE, USER_AGENT};
    use crate::authenticators::base::TokenResponse;

    const ibm_cloud_iam_url: &str = "ibm_cloud_iam_url";
    const api_key: &str = "apikey";
    const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

    #[test]
    fn set_urlencoded_parameter() {
        let token: Options = Options {
            grant_type: GRANT_TYPE.to_string(),
            apikey: api_key.to_string(),
        };
        let param = urlencoded_parameter(token);

        assert_eq!(param[0].0, "grant_type".to_string());
        assert_eq!(param[0].1, GRANT_TYPE.to_string());
        assert_eq!(param[1].0, "apikey".to_string());
        assert_eq!(param[1].0, api_key.to_string());
    }

    #[test]
    fn set_headers_map() {
        let headers = construct_headers();

        assert_eq!(headers.get(USER_AGENT).unwrap(), &"reqwest");
        assert_eq!(
            headers.get(CONTENT_TYPE).unwrap(),
            &"application/x-www-form-urlencoded"
        )
    }

    #[test]
    fn tokenresponse_get_access_token() {
        let tokenresponse = TokenResponse {
            access_token: "Token".to_string(),
            refresh_token: None,
            delegated_refresh_token: None,
            token_type: "".to_string(),
            expires_in: 0,
            expiration: 0,
        };
        assert_eq!(tokenresponse.get_access_token(), "Token".to_string())
    }

    #[test]
    fn tokenresponse_get_expiration() {
        let tokenresponse = TokenResponse {
            access_token: "".to_string(),
            refresh_token: None,
            delegated_refresh_token: None,
            token_type: "".to_string(),
            expires_in: 0,
            expiration: 169900991,
        };
        assert_eq!(tokenresponse.get_expiration(), 169900991)
    }
    #[test]
    fn tokenresponse_get_validate_token() {
        let invalid_token = TokenResponse {
            access_token: "".to_string(),
            refresh_token: None,
            delegated_refresh_token: None,
            token_type: "".to_string(),
            expires_in: 0,
            expiration: (Local::now().timestamp() - 3000) as i32,
        };

        let valid_token = TokenResponse {
            access_token: "".to_string(),
            refresh_token: None,
            delegated_refresh_token: None,
            token_type: "".to_string(),
            expires_in: 0,
            expiration: (Local::now().timestamp() + 3000) as i32,
        };

        assert_eq!(invalid_token.validate_token(), false);
        assert_eq!(valid_token.validate_token(), true)
    }
}
