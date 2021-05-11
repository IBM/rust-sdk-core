# AccountSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context** | Option<[**crate::models::ResponseContext**](ResponseContext.md)> |  | [optional]
**account_id** | **String** | Unique ID of the account. | 
**restrict_create_service_id** | **String** | Defines whether or not creating a Service Id is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value | [default to RestrictCreateServiceId_NOTSET]
**restrict_create_platform_apikey** | **String** | Defines whether or not creating platform API keys is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value | [default to RestrictCreatePlatformApikey_NOTSET]
**allowed_ip_addresses** | **String** | Defines the IP addresses and subnets from which IAM tokens can be created for the account. | 
**entity_tag** | **String** | Version of the account settings. | 
**mfa** | **String** | Defines the MFA trait for the account. Valid values:   * NONE - No MFA trait set   * TOTP - For all non-federated IBMId users   * TOTP4ALL - For all users   * LEVEL1 - Email-based MFA for all users   * LEVEL2 - TOTP-based MFA for all users   * LEVEL3 - U2F MFA for all users  | 
**history** | Option<[**Vec<crate::models::EnityHistoryRecord>**](EnityHistoryRecord.md)> | History of the Account Settings. | [optional]
**session_expiration_in_seconds** | **String** | Defines the session expiration in seconds for the account. Valid values:   * Any whole number between between '900' and '86400'   * NOT_SET - To unset account setting and use service default | [default to 86400]
**session_invalidation_in_seconds** | **String** | Defines the period of time in seconds in which a session will be invalidated due  to inactivity. Valid values:    * Any whole number between '900' and '7200'    * NOT_SET - To unset account setting and use service default | [default to 7200]
**max_sessions_per_identity** | **String** | Defines the max allowed sessions per identity required by the account. Valid values:   * Any whole number greater than 0   * NOT_SET - To unset account setting and use service default | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


