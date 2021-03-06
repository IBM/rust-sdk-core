/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateServiceIdRequest : Input body parameters for the Update service ID V1 REST request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateServiceIdRequest {
    /// The name of the service ID to update. If specified in the request the parameter must not be empty. The name is not checked for uniqueness. Failure to this will result in an Error condition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the service ID to update. If specified an empty description will clear the description of the service ID. If an non empty value is provided the service ID will be updated.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of CRNs which point to the services connected to this service ID. If specified an empty list will clear all existing unique instance crns of the service ID.
    #[serde(rename = "unique_instance_crns", skip_serializing_if = "Option::is_none")]
    pub unique_instance_crns: Option<Vec<String>>,
}

impl UpdateServiceIdRequest {
    /// Input body parameters for the Update service ID V1 REST request.
    pub fn new() -> UpdateServiceIdRequest {
        UpdateServiceIdRequest {
            name: None,
            description: None,
            unique_instance_crns: None,
        }
    }
}


