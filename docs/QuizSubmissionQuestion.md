# QuizSubmissionQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The ID of the QuizQuestion this answer is for. | 
**flagged** | Option<**bool**> | Whether this question is flagged. | [optional]
**answer** | Option<**String**> | The provided answer (if any) for this question. The format of this parameter depends on the type of the question, see the Appendix for more information. | [optional]
**answers** | Option<**Vec<String>**> | The possible answers for this question when those possible answers are necessary.  The presence of this parameter is dependent on permissions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


