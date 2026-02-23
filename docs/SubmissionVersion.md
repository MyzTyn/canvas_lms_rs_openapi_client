# SubmissionVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignment_id** | Option<**i32**> | the id of the assignment this submissions is for | [optional]
**assignment_name** | Option<**String**> | the name of the assignment this submission is for | [optional]
**body** | Option<**String**> | the body text of the submission | [optional]
**current_grade** | Option<**String**> | the most up to date grade for the current version of this submission | [optional]
**current_graded_at** | Option<**String**> | the latest time stamp for the grading of this submission | [optional]
**current_grader** | Option<**String**> | the name of the most recent grader for this submission | [optional]
**grade_matches_current_submission** | Option<**bool**> | boolean indicating whether the grade is equal to the current submission grade | [optional]
**graded_at** | Option<**String**> | time stamp for the grading of this version of the submission | [optional]
**grader** | Option<**String**> | the name of the user who graded this version of the submission | [optional]
**grader_id** | Option<**i32**> | the user id of the user who graded this version of the submission | [optional]
**id** | Option<**i32**> | the id of the submission of which this is a version | [optional]
**new_grade** | Option<**String**> | the updated grade provided in this version of the submission | [optional]
**new_graded_at** | Option<**String**> | the timestamp for the grading of this version of the submission (alias for graded_at) | [optional]
**new_grader** | Option<**String**> | alias for 'grader' | [optional]
**previous_grade** | Option<**String**> | the grade for the submission version immediately preceding this one | [optional]
**previous_graded_at** | Option<**String**> | the timestamp for the grading of the submission version immediately preceding this one | [optional]
**previous_grader** | Option<**String**> | the name of the grader who graded the version of this submission immediately preceding this one | [optional]
**score** | Option<**i32**> | the score for this version of the submission | [optional]
**user_name** | Option<**String**> | the name of the student who created this submission | [optional]
**submission_type** | Option<**String**> | the type of submission | [optional]
**url** | Option<**String**> | the url of the submission, if there is one | [optional]
**user_id** | Option<**i32**> | the user ID of the student who created this submission | [optional]
**workflow_state** | Option<**String**> | the state of the submission at this version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


