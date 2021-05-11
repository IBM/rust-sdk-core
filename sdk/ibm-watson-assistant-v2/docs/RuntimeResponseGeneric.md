# RuntimeResponseGeneric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** | The type of response returned by the dialog node. The specified response type must be supported by the client application or channel. | 
**text** | **String** | The text of the response. | 
**channels** | Option<[**Vec<crate::models::ResponseGenericChannel>**](ResponseGenericChannel.md)> | An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client. | [optional]
**time** | **i32** | How long to pause, in milliseconds. | 
**typing** | Option<**bool**> | Whether to send a \"user is typing\" event during the pause. | [optional]
**source** | **String** | The URL of the image. | 
**title** | **String** | The title or introductory text to show before the response. | 
**description** | Option<**String**> | The description to show with the the response. | [optional]
**preference** | Option<**String**> | The preferred type of control to display. | [optional]
**options** | [**Vec<crate::models::DialogNodeOutputOptionsElement>**](DialogNodeOutputOptionsElement.md) | An array of objects describing the options from which the user can choose. | 
**message_to_human_agent** | Option<**String**> | A message to be sent to the human agent who will be taking over the conversation. | [optional]
**agent_available** | Option<[**crate::models::AgentAvailabilityMessage**](AgentAvailabilityMessage.md)> |  | [optional]
**agent_unavailable** | Option<[**crate::models::AgentAvailabilityMessage**](AgentAvailabilityMessage.md)> |  | [optional]
**transfer_info** | [**crate::models::ChannelTransferInfo**](ChannelTransferInfo.md) |  | 
**topic** | Option<**String**> | A label identifying the topic of the conversation, derived from the **title** property of the relevant node or the **topic** property of the dialog node response. | [optional]
**suggestions** | [**Vec<crate::models::DialogSuggestion>**](DialogSuggestion.md) | An array of objects describing the possible matching dialog nodes from which the user can choose. | 
**message_to_user** | **String** | The message to display to the user when initiating a channel transfer. | 
**header** | **String** | The title or introductory text to show before the response. This text is defined in the search skill configuration. | 
**primary_results** | [**Vec<crate::models::SearchResult>**](SearchResult.md) | An array of objects that contains the search results to be displayed in the initial response to the user. | 
**additional_results** | [**Vec<crate::models::SearchResult>**](SearchResult.md) | An array of objects that contains additional search results that can be displayed to the user upon request. | 
**user_defined** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | An object containing any properties for the user-defined response type. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


