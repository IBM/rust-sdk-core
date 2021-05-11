# \SessionsApi

All URIs are relative to *https://api.us-south.assistant.watson.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_session**](SessionsApi.md#create_session) | **post** /v2/assistants/{assistant_id}/sessions | Create a session
[**delete_session**](SessionsApi.md#delete_session) | **delete** /v2/assistants/{assistant_id}/sessions/{session_id} | Delete session



## create_session

> crate::models::SessionResponse create_session(assistant_id, version)
Create a session

Create a new session. A session is used to send user input to a skill and receive responses. It also maintains the state of the conversation. A session persists until it is deleted, or until it times out because of inactivity. (For more information, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-settings).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** | Unique identifier of the assistant. To find the assistant ID in the Watson Assistant user interface, open the assistant settings and click **API Details**. For information about creating assistants, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-add#assistant-add-task).  **Note:** Currently, the v2 API does not support creating assistants. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |

### Return type

[**crate::models::SessionResponse**](SessionResponse.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_session

> serde_json::Value delete_session(assistant_id, version, session_id)
Delete session

Deletes a session explicitly before it times out. (For more information about the session inactivity timeout, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-settings)).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** | Unique identifier of the assistant. To find the assistant ID in the Watson Assistant user interface, open the assistant settings and click **API Details**. For information about creating assistants, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-add#assistant-add-task).  **Note:** Currently, the v2 API does not support creating assistants. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**session_id** | **String** | Unique identifier of the session. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

