# CreateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the API key. The name is not checked for uniqueness. Therefore multiple names with the same value can exist. Access is done via the UUID of the API key. | 
**description** | Option<**String**> | The optional description of the API key. The 'description' property is only available if a description was provided during a create of an API key. | [optional]
**iam_id** | **String** | The iam_id that this API key authenticates. | 
**account_id** | Option<**String**> | The account ID of the API key. | [optional]
**apikey** | Option<**String**> | You can optionally passthrough the API key value for this API key. If passed, NO validation of that apiKey value is done, i.e. the value can be non-URL safe. If omitted, the API key management will create an URL safe opaque API key value. The value of the API key is checked for uniqueness. Please ensure enough variations when passing in this value. | [optional]
**store_value** | Option<**bool**> | Send true or false to set whether the API key value is retrievable in the future by using the Get details of an API key request. If you create an API key for a user, you must specify `false` or omit the value. We don't allow storing of API keys for users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


