# MessageInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_type** | Option<**String**> | The type of user input. Currently, only text input is supported. | [optional][default to MessageType_Text]
**text** | Option<**String**> | The text of the user input. This string cannot contain carriage return, newline, or tab characters. | [optional]
**intents** | Option<[**Vec<crate::models::RuntimeIntent>**](RuntimeIntent.md)> | Intents to use when evaluating the user input. Include intents from the previous response to continue using those intents rather than trying to recognize intents in the new input. | [optional]
**entities** | Option<[**Vec<crate::models::RuntimeEntity>**](RuntimeEntity.md)> | Entities to use when evaluating the message. Include entities from the previous response to continue using those entities rather than detecting entities in the new input. | [optional]
**suggestion_id** | Option<**String**> | For internal use only. | [optional]
**options** | Option<[**crate::models::MessageInputOptions**](MessageInputOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


