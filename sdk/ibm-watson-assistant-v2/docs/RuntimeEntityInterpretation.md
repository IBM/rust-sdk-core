# RuntimeEntityInterpretation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**calendar_type** | Option<**String**> | The calendar used to represent a recognized date (for example, `Gregorian`). | [optional]
**datetime_link** | Option<**String**> | A unique identifier used to associate a recognized time and date. If the user input contains a date and time that are mentioned together (for example, `Today at 5`, the same **datetime_link** value is returned for both the `@sys-date` and `@sys-time` entities). | [optional]
**festival** | Option<**String**> | A locale-specific holiday name (such as `thanksgiving` or `christmas`). This property is included when a `@sys-date` entity is recognized based on a holiday name in the user input. | [optional]
**granularity** | Option<**String**> | The precision or duration of a time range specified by a recognized `@sys-time` or `@sys-date` entity. | [optional]
**range_link** | Option<**String**> | A unique identifier used to associate multiple recognized `@sys-date`, `@sys-time`, or `@sys-number` entities that are recognized as a range of values in the user's input (for example, `from July 4 until July 14` or `from 20 to 25`). | [optional]
**range_modifier** | Option<**String**> | The word in the user input that indicates that a `sys-date` or `sys-time` entity is part of an implied range where only one date or time is specified (for example, `since` or `until`). | [optional]
**relative_day** | Option<**f32**> | A recognized mention of a relative day, represented numerically as an offset from the current date (for example, `-1` for `yesterday` or `10` for `in ten days`). | [optional]
**relative_month** | Option<**f32**> | A recognized mention of a relative month, represented numerically as an offset from the current month (for example, `1` for `next month` or `-3` for `three months ago`). | [optional]
**relative_week** | Option<**f32**> | A recognized mention of a relative week, represented numerically as an offset from the current week (for example, `2` for `in two weeks` or `-1` for `last week). | [optional]
**relative_weekend** | Option<**f32**> | A recognized mention of a relative date range for a weekend, represented numerically as an offset from the current weekend (for example, `0` for `this weekend` or `-1` for `last weekend`). | [optional]
**relative_year** | Option<**f32**> | A recognized mention of a relative year, represented numerically as an offset from the current year (for example, `1` for `next year` or `-5` for `five years ago`). | [optional]
**specific_day** | Option<**f32**> | A recognized mention of a specific date, represented numerically as the date within the month (for example, `30` for `June 30`.) | [optional]
**specific_day_of_week** | Option<**String**> | A recognized mention of a specific day of the week as a lowercase string (for example, `monday`). | [optional]
**specific_month** | Option<**f32**> | A recognized mention of a specific month, represented numerically (for example, `7` for `July`). | [optional]
**specific_quarter** | Option<**f32**> | A recognized mention of a specific quarter, represented numerically (for example, `3` for `the third quarter`). | [optional]
**specific_year** | Option<**f32**> | A recognized mention of a specific year (for example, `2016`). | [optional]
**numeric_value** | Option<**f32**> | A recognized numeric value, represented as an integer or double. | [optional]
**subtype** | Option<**String**> | The type of numeric value recognized in the user input (`integer` or `rational`). | [optional]
**part_of_day** | Option<**String**> | A recognized term for a time that was mentioned as a part of the day in the user's input (for example, `morning` or `afternoon`). | [optional]
**relative_hour** | Option<**f32**> | A recognized mention of a relative hour, represented numerically as an offset from the current hour (for example, `3` for `in three hours` or `-1` for `an hour ago`). | [optional]
**relative_minute** | Option<**f32**> | A recognized mention of a relative time, represented numerically as an offset in minutes from the current time (for example, `5` for `in five minutes` or `-15` for `fifteen minutes ago`). | [optional]
**relative_second** | Option<**f32**> | A recognized mention of a relative time, represented numerically as an offset in seconds from the current time (for example, `10` for `in ten seconds` or `-30` for `thirty seconds ago`). | [optional]
**specific_hour** | Option<**f32**> | A recognized specific hour mentioned as part of a time value (for example, `10` for `10:15 AM`.) | [optional]
**specific_minute** | Option<**f32**> | A recognized specific minute mentioned as part of a time value (for example, `15` for `10:15 AM`.) | [optional]
**specific_second** | Option<**f32**> | A recognized specific second mentioned as part of a time value (for example, `30` for `10:15:30 AM`.) | [optional]
**timezone** | Option<**String**> | A recognized time zone mentioned as part of a time value (for example, `EST`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


