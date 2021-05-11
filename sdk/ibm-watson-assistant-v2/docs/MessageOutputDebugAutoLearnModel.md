# MessageOutputDebugAutoLearnModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**outcome** | Option<**String**> | Whether the model was consulted successfully. | [optional]
**model_type** | Option<**String**> | How the model was applied. | [optional]
**model_id** | Option<**String**> | Unique identifier of the autolearned model. | [optional]
**suggestions** | Option<[**Vec<crate::models::DialogSuggestion>**](DialogSuggestion.md)> | Possible responses the assistant would have returned with autolearning applied, either as disambiguation suggestions or alternate responses. Included only if the response was generated with autolearning in preview mode. (Preview mode means that autolearning is enabled, but is not being applied.) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


