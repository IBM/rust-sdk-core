# SearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the document in the Discovery service collection.  This property is included in responses from search skills, which are available only to Plus or Enterprise plan users. | 
**result_metadata** | [**crate::models::SearchResultMetadata**](SearchResultMetadata.md) |  | 
**body** | Option<**String**> | A description of the search result. This is taken from an abstract, summary, or highlight field in the Discovery service response, as specified in the search skill configuration. | [optional]
**title** | Option<**String**> | The title of the search result. This is taken from a title or name field in the Discovery service response, as specified in the search skill configuration. | [optional]
**url** | Option<**String**> | The URL of the original data object in its native data source. | [optional]
**highlight** | Option<[**crate::models::SearchResultHighlight**](SearchResultHighlight.md)> |  | [optional]
**answers** | Option<[**Vec<crate::models::SearchResultAnswer>**](SearchResultAnswer.md)> | An array specifying segments of text within the result that were identified as direct answers to the search query. Currently, only the single answer with the highest confidence (if any) is returned.  **Note:** This property uses the answer finding beta feature, and is available only if the search skill is connected to a Discovery v2 service instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


