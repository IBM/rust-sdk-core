# RuntimeEntityInterpretationSysTime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**datetime_link** | Option<**String**> | A unique identifier used to associate a recognized time and date. If the user input contains a date and time that are mentioned together (for example, `Today at 5`, the same **datetime_link** value is returned for both the `@sys-date` and `@sys-time` entities). | [optional]
**granularity** | Option<**String**> | The precision or duration of a time range specified by a recognized `@sys-time` or `@sys-date` entity. | [optional]
**part_of_day** | Option<**String**> | A recognized term for a time that was mentioned as a part of the day in the user's input (for example, `morning` or `afternoon`). | [optional]
**range_link** | Option<**String**> | A unique identifier used to associate multiple recognized `@sys-date`, `@sys-time`, or `@sys-number` entities that are recognized as a range of values in the user's input (for example, `from July 4 until July 14` or `from 20 to 25`). | [optional]
**relative_hour** | Option<**f32**> | A recognized mention of a relative hour, represented numerically as an offset from the current hour (for example, `3` for `in three hours` or `-1` for `an hour ago`). | [optional]
**relative_minute** | Option<**f32**> | A recognized mention of a relative time, represented numerically as an offset in minutes from the current time (for example, `5` for `in five minutes` or `-15` for `fifteen minutes ago`). | [optional]
**relative_second** | Option<**f32**> | A recognized mention of a relative time, represented numerically as an offset in seconds from the current time (for example, `10` for `in ten seconds` or `-30` for `thirty seconds ago`). | [optional]
**specific_hour** | Option<**f32**> | A recognized specific hour mentioned as part of a time value (for example, `10` for `10:15 AM`.) | [optional]
**specific_minute** | Option<**f32**> | A recognized specific minute mentioned as part of a time value (for example, `15` for `10:15 AM`.) | [optional]
**specific_second** | Option<**f32**> | A recognized specific second mentioned as part of a time value (for example, `30` for `10:15:30 AM`.) | [optional]
**timezone** | Option<**String**> | A recognized time zone mentioned as part of a time value (for example, `EST`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


