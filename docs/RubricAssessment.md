# RubricAssessment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the rubric | [optional]
**rubric_id** | Option<**i32**> | the rubric the assessment belongs to | [optional]
**rubric_association_id** | Option<**i32**> |  | [optional]
**score** | Option<**i32**> |  | [optional]
**artifact_type** | Option<**String**> | the object of the assessment | [optional]
**artifact_id** | Option<**i32**> | the id of the object of the assessment | [optional]
**artifact_attempt** | Option<**i32**> | the current number of attempts made on the object of the assessment | [optional]
**assessment_type** | Option<**String**> | the type of assessment. values will be either 'grading', 'peer_review', or 'provisional_grade' | [optional]
**assessor_id** | Option<**i32**> | user id of the person who made the assessment | [optional]
**data** | Option<**Vec<serde_json::Value>**> | (Optional) If 'full' is included in the 'style' parameter, returned assessments will have their full details contained in their data hash. If the user does not request a style, this key will be absent. | [optional]
**comments** | Option<**Vec<String>**> | (Optional) If 'comments_only' is included in the 'style' parameter, returned assessments will include only the comments portion of their data hash. If the user does not request a style, this key will be absent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


