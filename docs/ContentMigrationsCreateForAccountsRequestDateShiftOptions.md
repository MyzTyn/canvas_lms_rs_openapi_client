# ContentMigrationsCreateForAccountsRequestDateShiftOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shift_dates** | Option<**bool**> | [Boolean] Whether to shift dates in the copied course | [optional]
**old_start_date** | Option<**String**> | [Date] The original start date of the source content/course | [optional]
**old_end_date** | Option<**String**> | [Date] The original end date of the source content/course | [optional]
**new_start_date** | Option<**String**> | [Date] The new start date for the content/course | [optional]
**new_end_date** | Option<**String**> | [Date] The new end date for the source content/course | [optional]
**day_substitutions** | Option<**String**> | [X] [Integer] Move anything scheduled for day 'X' to the specified day. (0-Sunday, 1-Monday, 2-Tuesday, 3-Wednesday, 4-Thursday, 5-Friday, 6-Saturday) | [optional]
**remove_dates** | Option<**bool**> | [Boolean] Whether to remove dates in the copied course. Cannot be used in conjunction with *shift_dates*. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


