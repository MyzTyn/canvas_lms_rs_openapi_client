# GradeChangeEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the event. | [optional]
**created_at** | Option<**String**> | timestamp of the event | [optional]
**event_type** | Option<**String**> | GradeChange event type | [optional]
**excused_after** | Option<**bool**> | Boolean indicating whether the submission was excused after the change. | [optional]
**excused_before** | Option<**bool**> | Boolean indicating whether the submission was excused before the change. | [optional]
**grade_after** | Option<**String**> | The grade after the change. | [optional]
**grade_before** | Option<**String**> | The grade before the change. | [optional]
**graded_anonymously** | Option<**bool**> | Boolean indicating whether the student name was visible when the grade was given. Could be null if the grade change record was created before this feature existed. | [optional]
**version_number** | Option<**String**> | Version Number of the grade change submission. | [optional]
**request_id** | Option<**String**> | The unique request id of the request during the grade change. | [optional]
**links** | Option<[**models::GradeChangeEventLinks**](GradeChangeEventLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


