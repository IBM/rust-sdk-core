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


/// struct for typed errors of method `get_account_settings`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountSettingsError {
    Status400(),
    Status401(),
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_account_settings`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAccountSettingsError {
    Status400(),
    Status401(),
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// Returns the details of an account's configuration.
pub async fn get_account_settings(configuration: &configuration::Configuration, account_id: &str, authorization: &str, include_history: Option<bool>) -> Result<crate::models::AccountSettingsResponse, Error<GetAccountSettingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/accounts/{account_id}/settings/identity", configuration.base_path, account_id=crate::apis::urlencode(account_id));
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
        let local_var_entity: Option<GetAccountSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Allows a user to configure settings on their account with regards to MFA, session lifetimes,  access control for creating new identities, and enforcing IP restrictions on  token creation.
pub async fn update_account_settings(configuration: &configuration::Configuration, authorization: &str, if_match: &str, account_id: &str, account_settings_request: crate::models::AccountSettingsRequest) -> Result<crate::models::AccountSettingsResponse, Error<UpdateAccountSettingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/accounts/{account_id}/settings/identity", configuration.base_path, account_id=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("If-Match", if_match.to_string());
    local_var_req_builder = local_var_req_builder.json(&account_settings_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAccountSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

