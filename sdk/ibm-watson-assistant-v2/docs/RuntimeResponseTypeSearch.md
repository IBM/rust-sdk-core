# RuntimeResponseTypeSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** | The type of response returned by the dialog node. The specified response type must be supported by the client application or channel. | 
**header** | **String** | The title or introductory text to show before the response. This text is defined in the search skill configuration. | 
**primary_results** | [**Vec<crate::models::SearchResult>**](SearchResult.md) | An array of objects that contains the search results to be displayed in the initial response to the user. | 
**additional_results** | [**Vec<crate::models::SearchResult>**](SearchResult.md) | An array of objects that contains additional search results that can be displayed to the user upon request. | 
**channels** | Option<[**Vec<crate::models::ResponseGenericChannel>**](ResponseGenericChannel.md)> | An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


