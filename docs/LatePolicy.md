# LatePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the late policy | [optional]
**course_id** | **i32** | the unique identifier for the course | 
**missing_submission_deduction_enabled** | Option<**bool**> | whether to enable missing submission deductions | [optional]
**missing_submission_deduction** | Option<**f64**> | amount of percentage points to deduct | [optional]
**late_submission_deduction_enabled** | Option<**bool**> | whether to enable late submission deductions | [optional]
**late_submission_deduction** | Option<**f64**> | amount of percentage points to deduct per late_submission_interval | [optional]
**late_submission_interval** | Option<**String**> | time interval for late submission deduction | [optional]
**late_submission_minimum_percent_enabled** | Option<**bool**> | whether to enable late submission minimum percent | [optional]
**late_submission_minimum_percent** | Option<**f64**> | the minimum score a submission can receive in percentage points | [optional]
**created_at** | Option<**String**> | the time at which this late policy was originally created | [optional]
**updated_at** | Option<**String**> | the time at which this late policy was last modified in any way | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


