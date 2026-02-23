# GradebooksApplyScoreToUngradedSubmissionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**percent** | Option<**f64**> | A percentage value between 0 and 100 representing the percent score to apply. Exactly one of this parameter or the \"excused\" parameter (with a true value) must be specified. | [optional]
**excused** | Option<**bool**> | If true, mark ungraded submissions as excused. Exactly one of this parameter (with a true value) or the \"percent\" parameter must be specified. | [optional]
**mark_as_missing** | Option<**bool**> | If true, mark all affected submissions as missing in addition to issuing a grade. | [optional]
**only_past_due** | Option<**bool**> | If true, only operate on submissions whose due date has passed. | [optional]
**assignment_ids** | **Vec<String>** | An array of assignment ids to apply score to ungraded submissions. | 
**student_ids** | **Vec<String>** | An array of student ids to apply score to ungraded submissions. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


