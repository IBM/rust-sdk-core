# MessageOutputDebug

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nodes_visited** | Option<[**Vec<crate::models::DialogNodesVisited>**](DialogNodesVisited.md)> | An array of objects containing detailed diagnostic information about the nodes that were triggered during processing of the input message.  | [optional]
**log_messages** | Option<[**Vec<crate::models::DialogLogMessage>**](DialogLogMessage.md)> | An array of up to 50 messages logged with the request. | [optional]
**branch_exited** | Option<**bool**> | Assistant sets this to true when this message response concludes or interrupts a dialog. | [optional]
**branch_exited_reason** | Option<**String**> | When `branch_exited` is set to `true` by the Assistant, the `branch_exited_reason` specifies whether the dialog completed by itself or got interrupted. | [optional]
**auto_learn** | Option<[**serde_json::Value**](.md)> | For internal use only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


