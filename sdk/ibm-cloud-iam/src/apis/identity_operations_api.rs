/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `create_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApiKeyError {
    Status400(),
    Status401(),
    Status403(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApiKeyError {
    Status401(),
    Status403(),
    Status404(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiKeyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_api_keys_details`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiKeysDetailsError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_api_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApiKeysError {
    Status400(crate::models::ExceptionResponse),
    Status401(crate::models::ExceptionResponse),
    Status403(crate::models::ExceptionResponse),
    Status404(crate::models::ExceptionResponse),
    Status500(crate::models::ExceptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_service_ids`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServiceIdsError {
    Status400(),
    Status401(),
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `lock_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockApiKeyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `lock_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unlock_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlockApiKeyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unlock_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlockServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApiKeyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_service_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServiceIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// Creates an API key for a UserID or service ID. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to.   
pub async fn create_api_key(configuration: &configuration::Configuration, authorization: &str, create_api_key_request: crate::models::CreateApiKeyRequest, entity_lock: Option<&str>) -> Result<crate::models::ApiKey, Error<CreateApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    if let Some(local_var_param_value) = entity_lock {
        local_var_req_builder = local_var_req_builder.header("Entity-Lock", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&create_api_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a service ID for an IBM Cloud account. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn create_service_id(configuration: &configuration::Configuration, authorization: &str, create_service_id_request: crate::models::CreateServiceIdRequest, entity_lock: Option<&str>) -> Result<crate::models::ServiceId, Error<CreateServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    if let Some(local_var_param_value) = entity_lock {
        local_var_req_builder = local_var_req_builder.header("Entity-Lock", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&create_service_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes an API key. Existing tokens will remain valid until expired. Users can manage user API keys for themself, or service ID API  keys for service IDs that are bound to an entity they have access  to.   
pub async fn delete_api_key(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<DeleteApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a service ID and all API keys associated to it. Before deleting the service ID, all associated API keys are deleted. In case a Delete Conflict (status code 409) a retry of the request may help as the service ID is only deleted if the associated API keys were successfully deleted before. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn delete_service_id(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<DeleteServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the details of an API key. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to. In case of  service IDs and their API keys, a user must be either an account owner,  a IBM Cloud org manager or IBM Cloud space developer in order to manage  service IDs of the entity.
pub async fn get_api_key(configuration: &configuration::Configuration, id: &str, authorization: &str, include_history: Option<bool>) -> Result<crate::models::ApiKey, Error<GetApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_history {
        local_var_req_builder = local_var_req_builder.query(&[("include_history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the details of an API key by its value. Users can manage user API keys for themself, or service ID API keys  for service IDs that are bound to an entity they have access to.   
pub async fn get_api_keys_details(configuration: &configuration::Configuration, authorization: &str, iam_api_key: Option<&str>, include_history: Option<bool>) -> Result<crate::models::ApiKey, Error<GetApiKeysDetailsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/details", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_history {
        local_var_req_builder = local_var_req_builder.query(&[("include_history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = iam_api_key {
        local_var_req_builder = local_var_req_builder.header("IAM-ApiKey", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetApiKeysDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the details of a service ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn get_service_id(configuration: &configuration::Configuration, id: &str, authorization: &str, include_history: Option<bool>) -> Result<crate::models::ServiceId, Error<GetServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_history {
        local_var_req_builder = local_var_req_builder.query(&[("include_history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the list of API key details for a given service or user IAM ID and account ID. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to. In case of  service IDs and their API keys, a user must be either an account owner,  a IBM Cloud org manager or IBM Cloud space developer in order to manage  service IDs of the entity.
pub async fn list_api_keys(configuration: &configuration::Configuration, authorization: &str, account_id: Option<&str>, iam_id: Option<&str>, pagesize: Option<i32>, pagetoken: Option<&str>, scope: Option<&str>, _type: Option<&str>, sort: Option<&str>, order: Option<&str>, include_history: Option<bool>) -> Result<crate::models::ApiKeyList, Error<ListApiKeysError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account_id {
        local_var_req_builder = local_var_req_builder.query(&[("account_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = iam_id {
        local_var_req_builder = local_var_req_builder.query(&[("iam_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagesize {
        local_var_req_builder = local_var_req_builder.query(&[("pagesize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagetoken {
        local_var_req_builder = local_var_req_builder.query(&[("pagetoken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = scope {
        local_var_req_builder = local_var_req_builder.query(&[("scope", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _type {
        local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_history {
        local_var_req_builder = local_var_req_builder.query(&[("include_history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListApiKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of service IDs. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn list_service_ids(configuration: &configuration::Configuration, authorization: &str, account_id: Option<&str>, name: Option<&str>, pagesize: Option<i32>, pagetoken: Option<&str>, sort: Option<&str>, order: Option<&str>, include_history: Option<bool>) -> Result<crate::models::ServiceIdList, Error<ListServiceIdsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account_id {
        local_var_req_builder = local_var_req_builder.query(&[("account_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagesize {
        local_var_req_builder = local_var_req_builder.query(&[("pagesize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagetoken {
        local_var_req_builder = local_var_req_builder.query(&[("pagetoken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_history {
        local_var_req_builder = local_var_req_builder.query(&[("include_history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListServiceIdsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Locks an API key by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.
pub async fn lock_api_key(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<LockApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/{id}/lock", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LockApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Locks a service ID by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.
pub async fn lock_service_id(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<LockServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/{id}/lock", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LockServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unlocks an API key by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.
pub async fn unlock_api_key(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<UnlockApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/{id}/lock", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UnlockApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unlocks a service ID by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.
pub async fn unlock_service_id(configuration: &configuration::Configuration, id: &str, authorization: &str) -> Result<(), Error<UnlockServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/{id}/lock", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UnlockServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates properties of an API key. This does NOT affect existing access tokens. Their token content will stay unchanged until the access token is refreshed. To update an API key, pass the property to be modified. To delete one property's value, pass the property with an empty value \"\".Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn update_api_key(configuration: &configuration::Configuration, id: &str, if_match: &str, authorization: &str, update_api_key_request: crate::models::UpdateApiKeyRequest) -> Result<crate::models::ApiKey, Error<UpdateApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/apikeys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("If-Match", if_match.to_string());
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&update_api_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates properties of a service ID. This does NOT affect existing access tokens. Their token content will stay unchanged until the access token is refreshed. To update a service ID, pass the property to be modified. To delete one property's value, pass the property with an empty value \"\".Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   
pub async fn update_service_id(configuration: &configuration::Configuration, id: &str, if_match: &str, authorization: &str, update_service_id_request: crate::models::UpdateServiceIdRequest) -> Result<crate::models::ServiceId, Error<UpdateServiceIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/serviceids/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("If-Match", if_match.to_string());
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&update_service_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateServiceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

