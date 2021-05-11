# MessageContextGlobalSystem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timezone** | Option<**String**> | The user time zone. The assistant uses the time zone to correctly resolve relative time references. | [optional]
**user_id** | Option<**String**> | A string value that identifies the user who is interacting with the assistant. The client must provide a unique identifier for each individual end user who accesses the application. For user-based plans, this user ID is used to identify unique users for billing purposes. This string cannot contain carriage return, newline, or tab characters. If no value is specified in the input, **user_id** is automatically set to the value of **context.global.session_id**.  **Note:** This property is the same as the **user_id** property at the root of the message body. If **user_id** is specified in both locations in a message request, the value specified at the root is used. | [optional]
**turn_count** | Option<**i32**> | A counter that is automatically incremented with each turn of the conversation. A value of 1 indicates that this is the the first turn of a new conversation, which can affect the behavior of some skills (for example, triggering the start node of a dialog). | [optional]
**locale** | Option<**String**> | The language code for localization in the user input. The specified locale overrides the default for the assistant, and is used for interpreting entity values in user input such as date values. For example, `04/03/2018` might be interpreted either as April 3 or March 4, depending on the locale.   This property is included only if the new system entities are enabled for the skill. | [optional]
**reference_time** | Option<**String**> | The base time for interpreting any relative time mentions in the user input. The specified time overrides the current server time, and is used to calculate times mentioned in relative terms such as `now` or `tomorrow`. This can be useful for simulating past or future times for testing purposes, or when analyzing documents such as news articles.  This value must be a UTC time value formatted according to ISO 8601 (for example, `2019-06-26T12:00:00Z` for noon on 26 June 2019.  This property is included only if the new system entities are enabled for the skill. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


