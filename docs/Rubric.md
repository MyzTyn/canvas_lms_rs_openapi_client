# Rubric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the rubric | [optional]
**title** | Option<**String**> | title of the rubric | [optional]
**context_id** | Option<**i32**> | the context owning the rubric | [optional]
**context_type** | Option<**String**> |  | [optional]
**points_possible** | Option<**i32**> |  | [optional]
**reusable** | Option<**bool**> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**free_form_criterion_comments** | Option<**bool**> | whether or not free-form comments are used | [optional]
**hide_score_total** | Option<**bool**> |  | [optional]
**data** | Option<[**Vec<models::RubricCriterion>**](RubricCriterion.md)> | An array with all of this Rubric's grading Criteria | [optional]
**assessments** | Option<[**Vec<models::RubricAssessment>**](RubricAssessment.md)> | If an assessment type is included in the 'include' parameter, includes an array of rubric assessment objects for a given rubric, based on the assessment type requested. If the user does not request an assessment type this key will be absent. | [optional]
**associations** | Option<[**Vec<models::RubricAssociation>**](RubricAssociation.md)> | If an association type is included in the 'include' parameter, includes an array of rubric association objects for a given rubric, based on the association type requested. If the user does not request an association type this key will be absent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


