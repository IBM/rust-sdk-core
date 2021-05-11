# RuntimeResponseTypeImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** | The type of response returned by the dialog node. The specified response type must be supported by the client application or channel. | 
**source** | **String** | The URL of the image. | 
**title** | Option<**String**> | The title to show before the response. | [optional]
**description** | Option<**String**> | The description to show with the the response. | [optional]
**channels** | Option<[**Vec<crate::models::ResponseGenericChannel>**](ResponseGenericChannel.md)> | An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


