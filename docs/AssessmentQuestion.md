# AssessmentQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The ID of the assessment question. | 
**position** | Option<**i64**> | The order of the question. | [optional]
**assessment_question_bank_id** | Option<**i64**> | The ID of the question bank this question belongs to. | [optional]
**created_at** | Option<**String**> | The date and time when the assessment question was created. | [optional]
**question_name** | Option<**String**> | The name of the question. | [optional]
**question_type** | Option<**String**> | The type of the question. | [optional]
**question_text** | Option<**String**> | The text of the question. | [optional]
**points_possible** | Option<**f64**> | The maximum amount of points possible received for getting this question correct. | [optional]
**correct_comments** | Option<**String**> | The comments to display if the student answers the question correctly. | [optional]
**incorrect_comments** | Option<**String**> | The comments to display if the student answers incorrectly. | [optional]
**neutral_comments** | Option<**String**> | The comments to display regardless of how the student answered. | [optional]
**correct_comments_html** | Option<**String**> | The HTML version of the comments to display if the student answers the question correctly. | [optional]
**incorrect_comments_html** | Option<**String**> | The HTML version of the comments to display if the student answers incorrectly. | [optional]
**neutral_comments_html** | Option<**String**> | The HTML version of the comments to display regardless of how the student answered. | [optional]
**answers** | Option<[**Vec<models::AssessmentQuestionAnswersInner>**](AssessmentQuestionAnswersInner.md)> | An array of available answers. Each answer contains id, text, html, comments, comments_html, and weight properties. | [optional]
**variables** | Option<**Vec<serde_json::Value>**> | Variables for calculated questions. Null for other question types. | [optional]
**formulas** | Option<**Vec<serde_json::Value>**> | Formulas for calculated questions. Null for other question types. | [optional]
**answer_tolerance** | Option<**String**> | The tolerance for numerical answers. Null for non-numerical question types. | [optional]
**formula_decimal_places** | Option<**i32**> | The number of decimal places for formula results. Null for non-calculated question types. | [optional]
**matches** | Option<**Vec<serde_json::Value>**> | Matching pairs for matching questions. Null for other question types. | [optional]
**matching_answer_incorrect_matches** | Option<**Vec<serde_json::Value>**> | Incorrect match options for matching questions. Null for other question types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


