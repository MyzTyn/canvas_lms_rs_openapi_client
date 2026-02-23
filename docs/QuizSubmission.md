# QuizSubmission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The ID of the quiz submission. | 
**quiz_id** | **i64** | The ID of the Quiz the quiz submission belongs to. | 
**user_id** | Option<**i64**> | The ID of the Student that made the quiz submission. | [optional]
**submission_id** | Option<**i64**> | The ID of the Submission the quiz submission represents. | [optional]
**started_at** | Option<**String**> | The time at which the student started the quiz submission. | [optional]
**finished_at** | Option<**String**> | The time at which the student submitted the quiz submission. | [optional]
**end_at** | Option<**String**> | The time at which the quiz submission will be overdue, and be flagged as a late submission. | [optional]
**attempt** | Option<**i64**> | For quizzes that allow multiple attempts, this field specifies the quiz submission attempt number. | [optional]
**extra_attempts** | Option<**i64**> | Number of times the student was allowed to re-take the quiz over the multiple-attempt limit. | [optional]
**extra_time** | Option<**i64**> | Amount of extra time allowed for the quiz submission, in minutes. | [optional]
**manually_unlocked** | Option<**bool**> | The student can take the quiz even if it's locked for everyone else | [optional]
**time_spent** | Option<**i64**> | Amount of time spent, in seconds. | [optional]
**score** | Option<**i64**> | The score of the quiz submission, if graded. | [optional]
**score_before_regrade** | Option<**i64**> | The original score of the quiz submission prior to any re-grading. | [optional]
**kept_score** | Option<**i64**> | For quizzes that allow multiple attempts, this is the score that will be used, which might be the score of the latest, or the highest, quiz submission. | [optional]
**fudge_points** | Option<**i64**> | Number of points the quiz submission's score was fudged by. | [optional]
**has_seen_results** | Option<**bool**> | Whether the student has viewed their results to the quiz. | [optional]
**workflow_state** | Option<**String**> | The current state of the quiz submission. Possible values: ['untaken'|'pending_review'|'complete'|'settings_only'|'preview']. | [optional]
**overdue_and_needs_submission** | Option<**bool**> | Indicates whether the quiz submission is overdue and needs submission | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


