# RuntimeEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity** | **String** | An entity detected in the input. | 
**location** | **Vec<i32>** | An array of zero-based character offsets that indicate where the detected entity values begin and end in the input text. | 
**value** | **String** | The term in the input text that was recognized as an entity value. | 
**confidence** | Option<**f32**> | A decimal percentage that represents Watson's confidence in the recognized entity. | [optional]
**metadata** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Any metadata for the entity. | [optional]
**groups** | Option<[**Vec<crate::models::CaptureGroup>**](CaptureGroup.md)> | The recognized capture groups for the entity, as defined by the entity pattern. | [optional]
**interpretation** | Option<[**crate::models::RuntimeEntityInterpretation**](RuntimeEntityInterpretation.md)> |  | [optional]
**alternatives** | Option<[**Vec<crate::models::RuntimeEntityAlternative>**](RuntimeEntityAlternative.md)> | An array of possible alternative values that the user might have intended instead of the value returned in the **value** property. This property is returned only for `@sys-time` and `@sys-date` entities when the user's input is ambiguous.  This property is included only if the new system entities are enabled for the skill. | [optional]
**role** | Option<[**crate::models::RuntimeEntityRole**](RuntimeEntityRole.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


