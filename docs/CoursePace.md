# CoursePace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the course pace | [optional]
**course_id** | Option<**i32**> | the ID of the course | [optional]
**user_id** | Option<**i32**> | the ID of the user for this course pace | [optional]
**workflow_state** | Option<**String**> | the state of the course pace | [optional]
**exclude_weekends** | Option<**bool**> | boolean value depending on exclude weekends setting | [optional]
**selected_days_to_skip** | Option<**Vec<i32>**> | array of strings representing the days of the work week | [optional]
**hard_end_dates** | Option<**bool**> | set if the end date is set from course | [optional]
**created_at** | Option<**String**> | date when course pace is created | [optional]
**end_date** | Option<**String**> | course end date | [optional]
**updated_at** | Option<**String**> | date when course pace is updated | [optional]
**published_at** | Option<**String**> | date when course pace is published | [optional]
**root_account_id** | Option<**i32**> | the root account ID for this course pace | [optional]
**start_date** | Option<**String**> | course start date | [optional]
**modules** | Option<**Vec<serde_json::Value>**> | list of modules and items for this course pace | [optional]
**progress** | Option<[**models::Progress**](Progress.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


