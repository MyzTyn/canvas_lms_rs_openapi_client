# QuizExtension

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quiz_id** | **i64** | The ID of the Quiz the quiz extension belongs to. | 
**user_id** | **i64** | The ID of the Student that needs the quiz extension. | 
**extra_attempts** | Option<**i64**> | Number of times the student is allowed to re-take the quiz over the multiple-attempt limit. | [optional]
**extra_time** | Option<**i64**> | Amount of extra time allowed for the quiz submission, in minutes. | [optional]
**manually_unlocked** | Option<**bool**> | The student can take the quiz even if it's locked for everyone else | [optional]
**end_at** | Option<**String**> | The time at which the quiz submission will be overdue, and be flagged as a late submission. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


