/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApiKeyList : Response body format for the List API keys V1 REST request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyList {
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<crate::models::ResponseContext>>,
    /// The offset of the current page.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Optional size of a single page. Default is 20 items per page. Valid range is 1 to 100
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Link to the first page.
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    /// Link to the previous available page. If 'previous' property is not part of the response no previous page is available.
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    /// Link to the next available page. If 'next' property is not part of the response no next page is available.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// List of API keys based on the query paramters and the page size. The apikeys array is always part of the response but might be empty depending on the query parameters values provided.
    #[serde(rename = "apikeys")]
    pub apikeys: Vec<crate::models::ApiKey>,
}

impl ApiKeyList {
    /// Response body format for the List API keys V1 REST request.
    pub fn new(apikeys: Vec<crate::models::ApiKey>) -> ApiKeyList {
        ApiKeyList {
            context: None,
            offset: None,
            limit: None,
            first: None,
            previous: None,
            next: None,
            apikeys,
        }
    }
}


