# CoursePacesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_date** | Option<**String**> | End date of the course pace | [optional]
**end_date_context** | Option<**String**> | End date context (course, section, hupothetical) | [optional]
**start_date** | Option<**String**> | Start date of the course pace | [optional]
**start_date_context** | Option<**String**> | Start date context (course, section, hupothetical) | [optional]
**exclude_weekends** | Option<**bool**> | Course pace dates excludes weekends if true | [optional]
**hard_end_dates** | Option<**bool**> | Course pace uess hard end dates if true | [optional]
**workflow_state** | Option<**String**> | The state of the course pace | [optional]
**course_pace_module_item_attributes** | Option<**Vec<String>**> | [String] Module Items attributes | [optional]
**context_id** | Option<**i32**> | Pace Context ID | [optional]
**context_type** | Option<**String**> | Pace Context Type (Course, Section, User) | [optional]
**selected_days_to_skip** | Option<[**models::CoursePacesUpdateRequestSelectedDaysToSkip**](CoursePacesUpdateRequestSelectedDaysToSkip.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


