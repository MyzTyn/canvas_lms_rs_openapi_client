# GradeChangeEventLinks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignment** | Option<**i32**> | ID of the assignment associated with the event | [optional]
**course** | Option<**i32**> | ID of the course associated with the event. will match the context_id in the associated assignment if the context type for the assignment is a course | [optional]
**student** | Option<**i32**> | ID of the student associated with the event. will match the user_id in the associated submission. | [optional]
**grader** | Option<**i32**> | ID of the grader associated with the event. will match the grader_id in the associated submission. | [optional]
**page_view** | Option<**String**> | ID of the page view during the event if it exists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


