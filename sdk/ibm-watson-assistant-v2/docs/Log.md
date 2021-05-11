# Log

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_id** | **String** | A unique identifier for the logged event. | 
**request** | [**crate::models::MessageRequest**](MessageRequest.md) |  | 
**response** | [**crate::models::MessageResponse**](MessageResponse.md) |  | 
**assistant_id** | **String** | Unique identifier of the assistant. | 
**session_id** | **String** | The ID of the session the message was part of. | 
**skill_id** | **String** | The unique identifier of the skill that responded to the message. | 
**snapshot** | **String** | The name of the snapshot (dialog skill version) that responded to the message (for example, `draft`). | 
**request_timestamp** | **String** | The timestamp for receipt of the message. | 
**response_timestamp** | **String** | The timestamp for the system response to the message. | 
**language** | **String** | The language of the assistant to which the message request was made. | 
**customer_id** | Option<**String**> | The customer ID specified for the message, if any. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


