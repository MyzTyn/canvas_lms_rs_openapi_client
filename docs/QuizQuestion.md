# QuizQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The ID of the quiz question. | 
**quiz_id** | **i64** | The ID of the Quiz the question belongs to. | 
**assessment_question_bank_id** | Option<**i64**> | The ID of the assessment question bank this question belongs to. If assessment_question_bank_id has been enabled by SiteAdmin. | [optional]
**created_at** | Option<**String**> | The date and time when the quiz question was created. | [optional]
**position** | Option<**i64**> | The order in which the question will be retrieved and displayed. | [optional]
**question_name** | Option<**String**> | The name of the question. | [optional]
**question_type** | Option<**String**> | The type of the question. | [optional]
**question_text** | Option<**String**> | The text of the question. | [optional]
**points_possible** | Option<**i64**> | The maximum amount of points possible received for getting this question correct. | [optional]
**correct_comments** | Option<**String**> | The comments to display if the student answers the question correctly. | [optional]
**incorrect_comments** | Option<**String**> | The comments to display if the student answers incorrectly. | [optional]
**neutral_comments** | Option<**String**> | The comments to display regardless of how the student answered. | [optional]
**answers** | Option<[**Vec<models::Answer>**](Answer.md)> | An array of available answers to display to the student. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


