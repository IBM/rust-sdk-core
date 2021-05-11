# DialogNodeAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the action. | 
**_type** | Option<**String**> | The type of action to invoke. | [optional][default to Type_Client]
**parameters** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A map of key/value pairs to be provided to the action. | [optional]
**result_variable** | **String** | The location in the dialog context where the result of the action is stored. | 
**credentials** | Option<**String**> | The name of the context variable that the client application will use to pass in credentials for the action. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


