
use serde::Deserialize;
use chrono::Local;
use std::env;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseType {
    Ok(TokenResponse),
    Err(OidcExceptionResponse),
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenResponse {
    //Response body for POST /identity/token.

    //The IAM access token that can be used to invoke various IBM Cloud APIs.
    // Use this token with the prefix Bearer in the HTTP header Authorization
    // for invocations of IAM compatible APIs.
    pub(crate) access_token: String,

    //(optional) A refresh token that can be used to get a new IAM access
    // token if that token is expired. When using the default client
    // (no basic authorization header) as described in this documentation,
    // this refresh_token cannot be used to retrieve a new IAM access token.
    // When the IAM access token is about to be expired, use the API key to
    // create a new access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) refresh_token: Option<String>,

    //(optional) A delegated refresh token that can only be consumed by
    // the clients that have been specified in the API call as 'receiver_client_ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) delegated_refresh_token: Option<String>,

    //The type of the token. Currently, only Bearer is returned.
    pub(crate) token_type: String,

    // Number of seconds until the IAM access token will expire.
    pub(crate) expires_in: i32,

    // Number of seconds counted since January 1st, 1970, until the IAM access token will expire.
    pub(crate) expiration: i32,
}

impl TokenResponse {
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
    pub fn get_expiration(&self) -> i32 {
        self.expiration
    }
    pub(crate) fn validate_token(&self) -> bool {
        let local_time = Local::now().timestamp();
        let near_ex = self.get_expiration() as i64 - 5;
        if local_time >= near_ex {
            false
        } else {
            true
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExceptionResponseContext {
    //Context fill with key properties for problem determination.

    //The request ID of the inbound REST request.
    #[serde(rename = "requestId")]
    request_id: String,

    //The request type of the inbound REST request.
    #[serde(rename = "requestType")]
    request_type: String,

    //The user agent of the inbound REST request.
    #[serde(rename = "userAgent")]
    user_agent: String,

    // The URL of that cluster.
    url: String,

    //The instance ID of the server instance processing the request.
    #[serde(rename = "instanceId")]
    instance_id: String,

    //The thread ID of the server instance processing the request.
    #[serde(rename = "threadId")]
    thread_id: String,

    //The host of the server instance processing the request.
    host: String,

    //The start time of the request.
    #[serde(rename = "startTime")]
    start_time: String,

    //The finish time of the request.
    #[serde(rename = "endTime")]
    end_time: String,

    //The elapsed time in msec.
    #[serde(rename = "elapsedTime")]
    elapsed_time: String,

    //The language used to present the error message.
    locale: String,

    //The cluster name.
    #[serde(rename = "clusterName")]
    cluster_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MFARequirementsResponse {
    //Response properties for MFA requirements.

    //MFA error.
    error: String,

    //MFA Code.
    code: String,

    //MFA AuthorizationToken.
    authorizationToken: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OidcExceptionResponse {
    // Response body parameters in case of oidc error situations.

    // Context fill with key properties for problem determination.
    context: ExceptionResponseContext,

    //Error message code of the REST Exception.
    #[serde(rename = "errorCode")]
    error_code: String,

    //Error message of the REST Exception. Error messages are derived base on the input locale
    // of the REST request and the available Message catalogs. Dynamic fallback to 'us-english'
    // is happening if no message catalog is available for the provided input locale.
    #[serde(rename = "errorMessage")]
    error_message: String,
    //Error details of the REST Exception.

    //#[serde(rename = "errorDetails")]
    //error_details: String,

    //Response properties for MFA requirements.
    //#[serde(skip_deserializing)]
    //requirements: MFARequirementsResponse,
}

const IAM_CLOUD_URL_AUTH: &str = "https://iam.cloud.ibm.com/identity/token";

#[derive(Deserialize, Debug, Clone)]
pub struct Configs {
    pub(crate) IAM_IDENTITY_URL: String,
}

impl Configs {
    pub(crate) fn new() -> Configs {
        let key = "IAM_IDENTITY_URL";
        match env::var(key) {
            Ok(val) => Configs {
                IAM_IDENTITY_URL: val,
            },
            Err(..) => Configs {
                IAM_IDENTITY_URL: IAM_CLOUD_URL_AUTH.to_string(),
            },
        }
    }
}
