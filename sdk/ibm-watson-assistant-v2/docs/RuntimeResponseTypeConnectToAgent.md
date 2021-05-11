# RuntimeResponseTypeConnectToAgent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response_type** | **String** | The type of response returned by the dialog node. The specified response type must be supported by the client application or channel. | 
**message_to_human_agent** | Option<**String**> | A message to be sent to the human agent who will be taking over the conversation. | [optional]
**agent_available** | Option<[**crate::models::AgentAvailabilityMessage**](AgentAvailabilityMessage.md)> |  | [optional]
**agent_unavailable** | Option<[**crate::models::AgentAvailabilityMessage**](AgentAvailabilityMessage.md)> |  | [optional]
**transfer_info** | Option<[**crate::models::DialogNodeOutputConnectToAgentTransferInfo**](DialogNodeOutputConnectToAgentTransferInfo.md)> |  | [optional]
**topic** | Option<**String**> | A label identifying the topic of the conversation, derived from the **title** property of the relevant node or the **topic** property of the dialog node response. | [optional]
**channels** | Option<[**Vec<crate::models::ResponseGenericChannel>**](ResponseGenericChannel.md)> | An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


