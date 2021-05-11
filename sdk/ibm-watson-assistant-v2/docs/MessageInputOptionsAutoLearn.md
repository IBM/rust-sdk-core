# MessageInputOptionsAutoLearn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**learn** | Option<**bool**> | Whether the message should be used for autolearning. Specify `false` to exclude a message from autolearning (for example, if you are running tests on a production assistant). If autolearning is not enabled for the dialog skill, this option is ignored. | [optional][default to true]
**apply** | Option<**bool**> | Whether the autolearned model should be applied when responding to the message. You can use this option to compare responses with and without autolearning. If autolearning is not enabled for the dialog skill, this option is ignored. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


