# DialogSuggestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | The user-facing label for the suggestion. This label is taken from the **title** or **user_label** property of the corresponding dialog node, depending on the disambiguation options. | 
**value** | [**crate::models::DialogSuggestionValue**](DialogSuggestionValue.md) |  | 
**output** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The dialog output that will be returned from the Watson Assistant service if the user selects the corresponding option. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


