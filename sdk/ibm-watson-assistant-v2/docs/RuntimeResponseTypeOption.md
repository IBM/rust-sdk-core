# RuntimeResponseTypeOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** | The type of response returned by the dialog node. The specified response type must be supported by the client application or channel. | 
**title** | **String** | The title or introductory text to show before the response. | 
**description** | Option<**String**> | The description to show with the the response. | [optional]
**preference** | Option<**String**> | The preferred type of control to display. | [optional]
**options** | [**Vec<crate::models::DialogNodeOutputOptionsElement>**](DialogNodeOutputOptionsElement.md) | An array of objects describing the options from which the user can choose. | 
**channels** | Option<[**Vec<crate::models::ResponseGenericChannel>**](ResponseGenericChannel.md)> | An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


