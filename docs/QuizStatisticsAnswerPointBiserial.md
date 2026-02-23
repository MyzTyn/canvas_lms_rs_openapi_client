# QuizStatisticsAnswerPointBiserial

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**answer_id** | Option<**i64**> | ID of the answer the point biserial is for. | [optional]
**point_biserial** | Option<**f64**> | The point biserial value for this answer. Value ranges between -1 and 1. | [optional]
**correct** | Option<**bool**> | Convenience attribute that denotes whether this is the correct answer as opposed to being a distractor. This is mutually exclusive with the `distractor` value | [optional]
**distractor** | Option<**bool**> | Convenience attribute that denotes whether this is a distractor answer and not the correct one. This is mutually exclusive with the `correct` value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


