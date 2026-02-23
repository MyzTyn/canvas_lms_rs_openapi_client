# QuizStatisticsSubmissionStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unique_count** | Option<**i64**> | The number of students who have taken the quiz. | [optional]
**score_average** | Option<**f64**> | The mean of the student submission scores. | [optional]
**score_high** | Option<**f64**> | The highest submission score. | [optional]
**score_low** | Option<**f64**> | The lowest submission score. | [optional]
**score_stdev** | Option<**f64**> | Standard deviation of the submission scores. | [optional]
**scores** | Option<**serde_json::Value**> | A percentile distribution of the student scores, each key is the percentile (ranges between 0 and 100%) while the value is the number of students who received that score. | [optional]
**correct_count_average** | Option<**f64**> | The mean of the number of questions answered correctly by each student. | [optional]
**incorrect_count_average** | Option<**f64**> | The mean of the number of questions answered incorrectly by each student. | [optional]
**duration_average** | Option<**f64**> | The average time spent by students while taking the quiz. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


