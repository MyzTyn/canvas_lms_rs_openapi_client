# QuizAssignmentOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | ID of the assignment override, unless this is the base construct, in which case the 'id' field is omitted. | [optional]
**due_at** | Option<**String**> | The date after which any quiz submission is considered late. | [optional]
**unlock_at** | Option<**String**> | Date when the quiz becomes available for taking. | [optional]
**lock_at** | Option<**String**> | When the quiz will stop being available for taking. A value of null means it can always be taken. | [optional]
**title** | Option<**String**> | Title of the section this assignment override is for, if any. | [optional]
**base** | Option<**bool**> | If this property is present, it means that dates in this structure are not based on an assignment override, but are instead for all students. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


