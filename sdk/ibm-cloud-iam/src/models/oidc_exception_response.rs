/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OidcExceptionResponse : Response body parameters in case of oidc error situations.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OidcExceptionResponse {
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<crate::models::ExceptionResponseContext>>,
    /// Error message code of the REST Exception. 
    #[serde(rename = "errorCode")]
    pub error_code: String,
    /// Error message of the REST Exception. Error messages are derived base on the input locale of the REST request and the available Message catalogs. Dynamic fallback to 'us-english' is happening if no message catalog is available for the provided input locale.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    /// Error details of the REST Exception. 
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    #[serde(rename = "requirements", skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Box<crate::models::MfaRequirementsResponse>>,
}

impl OidcExceptionResponse {
    /// Response body parameters in case of oidc error situations.
    pub fn new(error_code: String, error_message: String) -> OidcExceptionResponse {
        OidcExceptionResponse {
            context: None,
            error_code,
            error_message,
            error_details: None,
            requirements: None,
        }
    }
}


