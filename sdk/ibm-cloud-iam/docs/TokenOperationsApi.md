# \TokenOperationsApi

All URIs are relative to *https://iam.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_token_api_key**](TokenOperationsApi.md#get_token_api_key) | **post** /identity/token#apikey | Create an IAM access token for a user or service ID using an API key
[**get_token_api_key_delegated_refresh_token**](TokenOperationsApi.md#get_token_api_key_delegated_refresh_token) | **post** /identity/token#apikey-delegated-refresh-token | Create an IAM access token and delegated refresh token for a user or service ID
[**get_token_iam_authz**](TokenOperationsApi.md#get_token_iam_authz) | **post** /identity/token#iam-authz | Create an IAM access token based on an authorization policy
[**get_token_password**](TokenOperationsApi.md#get_token_password) | **post** /identity/token#password | Create an IAM access token for a user using username / password credentials and an optional account identifier



## get_token_api_key

> crate::models::TokenResponse get_token_api_key(grant_type, apikey)
Create an IAM access token for a user or service ID using an API key

Creates a non-opaque access token for an API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | Grant type for this API call. You must set the grant type to `urn:ibm:params:oauth:grant-type:apikey`. | [required] |
**apikey** | **String** | The value of the api key. | [required] |

### Return type

[**crate::models::TokenResponse**](token-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_api_key_delegated_refresh_token

> crate::models::TokenResponse get_token_api_key_delegated_refresh_token(grant_type, apikey, response_type, receiver_client_ids, delegated_refresh_token_expiry)
Create an IAM access token and delegated refresh token for a user or service ID

Creates a non-opaque access token and a delegated refresh token for an API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | Grant type for this API call. You must set the grant type to `urn:ibm:params:oauth:grant-type:apikey`. | [required] |
**apikey** | **String** | The value of the API key. | [required] |
**response_type** | **String** | Either 'delegated_refresh_token' to receive a delegated refresh token only, or 'cloud_iam delegated_refresh_token' to receive both an IAM access token and a delegated refresh token in one API call. | [required] |
**receiver_client_ids** | **String** | A comma separated list of one or more client IDs that will be able to consume the delegated refresh token. The service that accepts a delegated refresh token as API parameter must expose its client ID to allow this API call. The receiver of the delegated refresh token will be able to use the refresh token until it expires. | [required] |
**delegated_refresh_token_expiry** | Option<**i32**> | Expiration in seconds until the delegated refresh token must be consumed by the receiver client IDs. After the expiration, no client ID can consume the delegated refresh token, even if the life time of the refresh token inside is still not expired. The default, if not specified, is 518,400 seconds which corresponds to 6 days. |  |

### Return type

[**crate::models::TokenResponse**](token-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_iam_authz

> crate::models::TokenResponse get_token_iam_authz(grant_type, access_token, desired_iam_id)
Create an IAM access token based on an authorization policy

Creates a non-opaque access token, if an appropriate authorization policy is in place. This kind of IAM access token is typically used for access between services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | Grant type for this API call. You must set the grant type to `urn:ibm:params:oauth:grant-type:iam-authz`. | [required] |
**access_token** | **String** | The IAM access token of the identity that has the appropriate authorization to create an IAM access token for a given resource. | [required] |
**desired_iam_id** | **String** | The IAM ID of the IAM access token identity that should be created. The desired_iam_id identifies a resource identity. The IAM ID consists of the prefix crn- and the CRN of the target identity, e.g. crn-crn:v1:bluemix:public:cloud-object-storage:global:a/59bcbfa6ea2f006b4ed7094c1a08dcdd:1a0ec336-f391-4091-a6fb-5e084a4c56f4::. | [required] |

### Return type

[**crate::models::TokenResponse**](token-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_password

> crate::models::TokenResponse get_token_password(authorization, grant_type, username, password, account)
Create an IAM access token for a user using username / password credentials and an optional account identifier

Creates a non-opaque access token for a username and password. To be able to call IBM Cloud APIs, the token must be made account-specific. For this purpose, also pass the 32 character long identifier for your account in the API call. This API call is possible only for non-federated IBMid users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Basic Authorization Header containing a valid client ID and secret. If this header is omitted the request fails with BXNIM0308E: 'No authorization header found'. You can use the client ID and secret that is used by the IBM Cloud CLI: `bx / bx` | [required] |
**grant_type** | **String** | Grant type for this API call. You must set the grant type to `password`. | [required] |
**username** | **String** | The value of the username. | [required] |
**password** | **String** | The value of the password. | [required] |
**account** | Option<**String**> | The 32 character identifier of the account. Specify this parameter to get an account-specific IAM token. IBM Cloud APIs require that IAM tokens are account-specific. |  |

### Return type

[**crate::models::TokenResponse**](token-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

