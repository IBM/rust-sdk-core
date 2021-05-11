# \MessageApi

All URIs are relative to *https://api.us-south.assistant.watson.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**message**](MessageApi.md#message) | **post** /v2/assistants/{assistant_id}/sessions/{session_id}/message | Send user input to assistant (stateful)
[**message_stateless**](MessageApi.md#message_stateless) | **post** /v2/assistants/{assistant_id}/message | Send user input to assistant (stateless)



## message

> crate::models::MessageResponse message(assistant_id, session_id, version, request)
Send user input to assistant (stateful)

Send user input to an assistant and receive a response, with conversation state (including context data) stored by Watson Assistant for the duration of the session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** | Unique identifier of the assistant. To find the assistant ID in the Watson Assistant user interface, open the assistant settings and click **API Details**. For information about creating assistants, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-add#assistant-add-task).  **Note:** Currently, the v2 API does not support creating assistants. | [required] |
**session_id** | **String** | Unique identifier of the session. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**request** | Option<[**MessageRequest**](MessageRequest.md)> | The message to be sent. This includes the user's input, along with optional content such as intents and entities. |  |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_stateless

> crate::models::MessageResponseStateless message_stateless(assistant_id, version, request)
Send user input to assistant (stateless)

Send user input to an assistant and receive a response, with conversation state (including context data) managed by your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** | Unique identifier of the assistant. To find the assistant ID in the Watson Assistant user interface, open the assistant settings and click **API Details**. For information about creating assistants, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-add#assistant-add-task).  **Note:** Currently, the v2 API does not support creating assistants. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**request** | Option<[**MessageRequestStateless**](MessageRequestStateless.md)> | The message to be sent. This includes the user's input, context data, and optional content such as intents and entities. |  |

### Return type

[**crate::models::MessageResponseStateless**](MessageResponseStateless.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

