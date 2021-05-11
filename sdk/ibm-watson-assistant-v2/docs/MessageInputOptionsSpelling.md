# MessageInputOptionsSpelling

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**suggestions** | Option<**bool**> | Whether to use spelling correction when processing the input. If spelling correction is used and **auto_correct** is `true`, any spelling corrections are automatically applied to the user input. If **auto_correct** is `false`, any suggested corrections are returned in the **output.spelling** property.  This property overrides the value of the **spelling_suggestions** property in the workspace settings for the skill. | [optional]
**auto_correct** | Option<**bool**> | Whether to use autocorrection when processing the input. If this property is `true`, any corrections are automatically applied to the user input, and the original text is returned in the **output.spelling** property of the message response. This property overrides the value of the **spelling_auto_correct** property in the workspace settings for the skill. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


