# MessageOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**generic** | Option<[**Vec<crate::models::RuntimeResponseGeneric>**](RuntimeResponseGeneric.md)> | Output intended for any channel. It is the responsibility of the client application to implement the supported response types. | [optional]
**intents** | Option<[**Vec<crate::models::RuntimeIntent>**](RuntimeIntent.md)> | An array of intents recognized in the user input, sorted in descending order of confidence | [optional]
**entities** | Option<[**Vec<crate::models::RuntimeEntity>**](RuntimeEntity.md)> | An array of entities identified in the user input | [optional]
**actions** | Option<[**Vec<crate::models::DialogNodeAction>**](DialogNodeAction.md)> | An array of objects describing any actions requested by the dialog node. | [optional]
**debug** | Option<[**crate::models::MessageOutputDebug**](MessageOutputDebug.md)> |  | [optional]
**user_defined** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object containing any custom properties included in the response. This object includes any arbitrary properties defined in the dialog JSON editor as part of the dialog node output. | [optional]
**spelling** | Option<[**crate::models::MessageOutputSpelling**](MessageOutputSpelling.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


