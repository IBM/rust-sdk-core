# \UserDataApi

All URIs are relative to *https://api.us-south.assistant.watson.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_data**](UserDataApi.md#delete_user_data) | **delete** /v2/user_data | Delete labeled data



## delete_user_data

> serde_json::Value delete_user_data(version, customer_id)
Delete labeled data

Deletes all data associated with a specified customer ID. The method has no effect if no data is associated with the customer ID.   You associate a customer ID with data by passing the `X-Watson-Metadata` header with a request that passes data. For more information about personal data and customer IDs, see [Information security](https://cloud.ibm.com/docs/assistant?topic=assistant-information-security#information-security).  **Note:** This operation is intended only for deleting data associated with a single specific customer, not for deleting data associated with multiple customers or for any other purpose. For more information, see [Labeling and deleting data in Watson Assistant](https://cloud.ibm.com/docs/assistant?topic=assistant-information-security#information-security-gdpr-wa).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**customer_id** | **String** | The customer ID for which all data is to be deleted. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

