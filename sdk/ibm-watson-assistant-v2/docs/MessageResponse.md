# MessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**output** | [**crate::models::MessageOutput**](MessageOutput.md) |  | 
**context** | Option<[**crate::models::MessageContext**](MessageContext.md)> |  | [optional]
**user_id** | **String** | A string value that identifies the user who is interacting with the assistant. The client must provide a unique identifier for each individual end user who accesses the application. For user-based plans, this user ID is used to identify unique users for billing purposes. This string cannot contain carriage return, newline, or tab characters. If no value is specified in the input, **user_id** is automatically set to the value of **context.global.session_id**.  **Note:** This property is the same as the **user_id** property in the global system context. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


