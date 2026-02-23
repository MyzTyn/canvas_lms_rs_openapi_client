# Score

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The lti_user_id or the Canvas user_id | [optional]
**score_given** | Option<**f64**> | The Current score received in the tool for this line item and user, scaled to the scoreMaximum | [optional]
**score_maximum** | Option<**f64**> | Maximum possible score for this result; it must be present if scoreGiven is present. | [optional]
**comment** | Option<**String**> | Comment visible to the student about this score. | [optional]
**timestamp** | Option<**String**> | Date and time when the score was modified in the tool. Should use subsecond precision. | [optional]
**activity_progress** | Option<**String**> | Indicate to Canvas the status of the user towards the activity's completion. Must be one of Initialized, Started, InProgress, Submitted, Completed | [optional]
**grading_progress** | Option<**String**> | Indicate to Canvas the status of the grading process. A value of PendingManual will require intervention by a grader. Values of NotReady, Failed, and Pending will cause the scoreGiven to be ignored. FullyGraded values will require no action. Possible values are NotReady, Failed, Pending, PendingManual, FullyGraded | [optional]
**submission** | Option<**serde_json::Value**> | Contains metadata about the submission attempt, like submittedAt: Date and time that the submission was originally created - should use ISO8601-formatted date with subsecond precision. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


