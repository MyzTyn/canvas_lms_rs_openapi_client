# RecordAlreadyExistsCreateRequestLatePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**missing_submission_deduction_enabled** | Option<**bool**> | [Boolean] Whether to enable the missing submission deduction late policy. | [optional]
**missing_submission_deduction** | Option<**f64**> | [Number] How many percentage points to deduct from a missing submission. | [optional]
**late_submission_deduction_enabled** | Option<**bool**> | [Boolean] Whether to enable the late submission deduction late policy. | [optional]
**late_submission_deduction** | Option<**f64**> | [Number] How many percentage points to deduct per the late submission interval. | [optional]
**late_submission_interval** | Option<**String**> | [String] The interval for late policies. | [optional]
**late_submission_minimum_percent_enabled** | Option<**bool**> | [Boolean] Whether to enable the late submission minimum percent for a late policy. | [optional]
**late_submission_minimum_percent** | Option<**f64**> | [Number] The minimum grade a submissions can have in percentage points. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


