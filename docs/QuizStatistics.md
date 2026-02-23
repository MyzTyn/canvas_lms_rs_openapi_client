# QuizStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The ID of the quiz statistics report. | 
**quiz_id** | **i64** | The ID of the Quiz the statistics report is for.  NOTE: AVAILABLE ONLY IN NON-JSON-API REQUESTS. | 
**multiple_attempts_exist** | Option<**bool**> | Whether there are any students that have made mutliple submissions for this quiz. | [optional]
**includes_all_versions** | Option<**bool**> | In the presence of multiple attempts, this field describes whether the statistics describe all the submission attempts and not only the latest ones. | [optional]
**generated_at** | Option<**String**> | The time at which the statistics were generated, which is usually after the occurrence of a quiz event, like a student submitting it. | [optional]
**url** | Option<**String**> | The API HTTP/HTTPS URL to this quiz statistics. | [optional]
**html_url** | Option<**String**> | The HTTP/HTTPS URL to the page where the statistics can be seen visually. | [optional]
**question_statistics** | Option<[**models::QuizStatisticsQuestionStatistics**](QuizStatisticsQuestionStatistics.md)> |  | [optional]
**submission_statistics** | Option<[**models::QuizStatisticsSubmissionStatistics**](QuizStatisticsSubmissionStatistics.md)> |  | [optional]
**links** | Option<[**models::QuizStatisticsLinks**](QuizStatisticsLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


