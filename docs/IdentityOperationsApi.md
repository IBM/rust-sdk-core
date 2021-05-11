# \IdentityOperationsApi

All URIs are relative to *https://iam.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](IdentityOperationsApi.md#create_api_key) | **post** /v1/apikeys | Create an API key
[**create_service_id**](IdentityOperationsApi.md#create_service_id) | **post** /v1/serviceids/ | Create a service ID
[**delete_api_key**](IdentityOperationsApi.md#delete_api_key) | **delete** /v1/apikeys/{id} | Deletes an API key
[**delete_service_id**](IdentityOperationsApi.md#delete_service_id) | **delete** /v1/serviceids/{id} | Deletes a service ID and associated API keys
[**get_api_key**](IdentityOperationsApi.md#get_api_key) | **get** /v1/apikeys/{id} | Get details of an API key
[**get_api_keys_details**](IdentityOperationsApi.md#get_api_keys_details) | **get** /v1/apikeys/details | Get details of an API key by its value
[**get_service_id**](IdentityOperationsApi.md#get_service_id) | **get** /v1/serviceids/{id} | Get details of a service ID
[**list_api_keys**](IdentityOperationsApi.md#list_api_keys) | **get** /v1/apikeys | Get API keys for a given service or user IAM ID and account ID
[**list_service_ids**](IdentityOperationsApi.md#list_service_ids) | **get** /v1/serviceids/ | List service IDs
[**lock_api_key**](IdentityOperationsApi.md#lock_api_key) | **post** /v1/apikeys/{id}/lock | Lock the API key
[**lock_service_id**](IdentityOperationsApi.md#lock_service_id) | **post** /v1/serviceids/{id}/lock | Lock the service ID
[**unlock_api_key**](IdentityOperationsApi.md#unlock_api_key) | **delete** /v1/apikeys/{id}/lock | Unlock the API key
[**unlock_service_id**](IdentityOperationsApi.md#unlock_service_id) | **delete** /v1/serviceids/{id}/lock | Unlock the service ID
[**update_api_key**](IdentityOperationsApi.md#update_api_key) | **put** /v1/apikeys/{id} | Updates an API key
[**update_service_id**](IdentityOperationsApi.md#update_service_id) | **put** /v1/serviceids/{id} | Update service ID



## create_api_key

> crate::models::ApiKey create_api_key(authorization, create_api_key_request, entity_lock)
Create an API key

Creates an API key for a UserID or service ID. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) | Request to create an API key. | [required] |
**entity_lock** | Option<**String**> | Indicates if the API key is locked for further write operations. False by default. |  |[default to false]

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service_id

> crate::models::ServiceId create_service_id(authorization, create_service_id_request, entity_lock)
Create a service ID

Creates a service ID for an IBM Cloud account. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**create_service_id_request** | [**CreateServiceIdRequest**](CreateServiceIdRequest.md) | Request to create a service ID. | [required] |
**entity_lock** | Option<**String**> | Indicates if the service ID is locked for further write operations. False by default. |  |[default to false]

### Return type

[**crate::models::ServiceId**](ServiceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(id, authorization)
Deletes an API key

Deletes an API key. Existing tokens will remain valid until expired. Users can manage user API keys for themself, or service ID API  keys for service IDs that are bound to an entity they have access  to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the API key. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_id

> delete_service_id(id, authorization)
Deletes a service ID and associated API keys

Deletes a service ID and all API keys associated to it. Before deleting the service ID, all associated API keys are deleted. In case a Delete Conflict (status code 409) a retry of the request may help as the service ID is only deleted if the associated API keys were successfully deleted before. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the service ID. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> crate::models::ApiKey get_api_key(id, authorization, include_history)
Get details of an API key

Returns the details of an API key. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to. In case of  service IDs and their API keys, a user must be either an account owner,  a IBM Cloud org manager or IBM Cloud space developer in order to manage  service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the API key. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**include_history** | Option<**bool**> | Defines if the entity history is included in the response. |  |[default to false]

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys_details

> crate::models::ApiKey get_api_keys_details(authorization, iam_api_key, include_history)
Get details of an API key by its value

Returns the details of an API key by its value. Users can manage user API keys for themself, or service ID API keys  for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**iam_api_key** | Option<**String**> | API key value. |  |
**include_history** | Option<**bool**> | Defines if the entity history is included in the response. |  |[default to false]

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_id

> crate::models::ServiceId get_service_id(id, authorization, include_history)
Get details of a service ID

Returns the details of a service ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the service ID. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**include_history** | Option<**bool**> | Defines if the entity history is included in the response. |  |[default to false]

### Return type

[**crate::models::ServiceId**](ServiceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> crate::models::ApiKeyList list_api_keys(authorization, account_id, iam_id, pagesize, pagetoken, scope, _type, sort, order, include_history)
Get API keys for a given service or user IAM ID and account ID

Returns the list of API key details for a given service or user IAM ID and account ID. Users can manage user API keys for themself, or service ID API keys for  service IDs that are bound to an entity they have access to. In case of  service IDs and their API keys, a user must be either an account owner,  a IBM Cloud org manager or IBM Cloud space developer in order to manage  service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**account_id** | Option<**String**> | Account ID of the API keys(s) to query. If a service IAM ID is specified in iam_id then account_id must match the account of the IAM ID. If a user IAM ID is specified in iam_id then then account_id must match the account of the Authorization token. |  |
**iam_id** | Option<**String**> | IAM ID of the API key(s) to be queried. The IAM ID may be that of a user or a service. For a user IAM ID iam_id must match the Authorization token. |  |
**pagesize** | Option<**i32**> | Optional size of a single page. Default is 20 items per page. Valid range is 1 to 100. |  |
**pagetoken** | Option<**String**> | Optional Prev or Next page token returned from a previous query execution. Default is start with first page. |  |
**scope** | Option<**String**> | Optional parameter to define the scope of the queried API Keys. Can be 'entity' (default) or 'account'. |  |[default to entity]
**_type** | Option<**String**> | Optional parameter to filter the type of the queried API Keys. Can be 'user' or 'serviceid'. |  |
**sort** | Option<**String**> | Optional sort property, valid values are name, description, created_at and created_by. If specified, the items are sorted by the value of this property. |  |
**order** | Option<**String**> | Optional sort order, valid values are asc and desc. Default: asc. |  |[default to asc]
**include_history** | Option<**bool**> | Defines if the entity history is included in the response. |  |[default to false]

### Return type

[**crate::models::ApiKeyList**](ApiKeyList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_ids

> crate::models::ServiceIdList list_service_ids(authorization, account_id, name, pagesize, pagetoken, sort, order, include_history)
List service IDs

Returns a list of service IDs. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**account_id** | Option<**String**> | Account ID of the service ID(s) to query. This parameter is required (unless using a pagetoken). |  |
**name** | Option<**String**> | Name of the service ID(s) to query. Optional.20 items per page. Valid range is 1 to 100. |  |
**pagesize** | Option<**i32**> | Optional size of a single page. Default is 20 items per page. Valid range is 1 to 100. |  |
**pagetoken** | Option<**String**> | Optional Prev or Next page token returned from a previous query execution. Default is start with first page. |  |
**sort** | Option<**String**> | Optional sort property, valid values are name, description, created_at and modified_at. If specified, the items are sorted by the value of this property. |  |
**order** | Option<**String**> | Optional sort order, valid values are asc and desc. Default: asc. |  |[default to asc]
**include_history** | Option<**bool**> | Defines if the entity history is included in the response. |  |[default to false]

### Return type

[**crate::models::ServiceIdList**](ServiceIdList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_api_key

> lock_api_key(id, authorization)
Lock the API key

Locks an API key by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the API key. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_service_id

> lock_service_id(id, authorization)
Lock the service ID

Locks a service ID by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the service ID. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_api_key

> unlock_api_key(id, authorization)
Unlock the API key

Unlocks an API key by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the API key. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_service_id

> unlock_service_id(id, authorization)
Unlock the service ID

Unlocks a service ID by ID. Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to. In case of service IDs and their API keys, a user must be either an account owner, a IBM Cloud org manager or IBM Cloud space developer in order to manage service IDs of the entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the service ID. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> crate::models::ApiKey update_api_key(id, if_match, authorization, update_api_key_request)
Updates an API key

Updates properties of an API key. This does NOT affect existing access tokens. Their token content will stay unchanged until the access token is refreshed. To update an API key, pass the property to be modified. To delete one property's value, pass the property with an empty value \"\".Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the API key to be updated. | [required] |
**if_match** | **String** | Version of the API key to be updated. Specify the version that you retrieved when reading the API key. This value  helps identifying parallel usage of this API. Pass * to indicate to update any version available. This might result in stale updates.  | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md) | Request to update an API key. | [required] |

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_id

> crate::models::ServiceId update_service_id(id, if_match, authorization, update_service_id_request)
Update service ID

Updates properties of a service ID. This does NOT affect existing access tokens. Their token content will stay unchanged until the access token is refreshed. To update a service ID, pass the property to be modified. To delete one property's value, pass the property with an empty value \"\".Users can manage user API keys for themself, or service ID API keys for service IDs that are bound to an entity they have access to.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the service ID to be updated. | [required] |
**if_match** | **String** | Version of the service ID to be updated. Specify the version that you retrieved as entity_tag (ETag header) when reading the service ID. This value helps identifying parallel usage of this API. Pass * to indicate to update any version available. This might result in stale updates. | [required] |
**authorization** | **String** | Authorization Token used for the request. The supported token type is a Cloud IAM Access Token. If the token is omitted the request will fail with BXNIM0308E: 'No authorization header found'. Please make sure that the provided token has the required authority for the request. | [required] |
**update_service_id_request** | [**UpdateServiceIdRequest**](UpdateServiceIdRequest.md) | Request to update a service ID. | [required] |

### Return type

[**crate::models::ServiceId**](ServiceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

