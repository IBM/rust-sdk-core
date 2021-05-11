# MessageInputOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restart** | Option<**bool**> | Whether to restart dialog processing at the root of the dialog, regardless of any previously visited nodes. **Note:** This does not affect `turn_count` or any other context variables. | [optional][default to false]
**alternate_intents** | Option<**bool**> | Whether to return more than one intent. Set to `true` to return all matching intents. | [optional][default to false]
**spelling** | Option<[**crate::models::MessageInputOptionsSpelling**](MessageInputOptionsSpelling.md)> |  | [optional]
**auto_learn** | Option<[**crate::models::MessageInputOptionsAutoLearn**](MessageInputOptionsAutoLearn.md)> |  | [optional]
**debug** | Option<**bool**> | Whether to return additional diagnostic information. Set to `true` to return additional information in the `output.debug` property. If you also specify **return_context**=`true`, the returned skill context includes the `system.state` property. | [optional][default to false]
**return_context** | Option<**bool**> | Whether to return session context with the response. If you specify `true`, the response includes the `context` property. If you also specify **debug**=`true`, the returned skill context includes the `system.state` property. | [optional][default to false]
**export** | Option<**bool**> | Whether to return session context, including full conversation state. If you specify `true`, the response includes the `context` property, and the skill context includes the `system.state` property.  **Note:** If **export**=`true`, the context is returned regardless of the value of **return_context**. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


