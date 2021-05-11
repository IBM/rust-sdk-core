# BaseMessageInputOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restart** | Option<**bool**> | Whether to restart dialog processing at the root of the dialog, regardless of any previously visited nodes. **Note:** This does not affect `turn_count` or any other context variables. | [optional][default to false]
**alternate_intents** | Option<**bool**> | Whether to return more than one intent. Set to `true` to return all matching intents. | [optional][default to false]
**spelling** | Option<[**crate::models::MessageInputOptionsSpelling**](MessageInputOptionsSpelling.md)> |  | [optional]
**auto_learn** | Option<[**crate::models::MessageInputOptionsAutoLearn**](MessageInputOptionsAutoLearn.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


