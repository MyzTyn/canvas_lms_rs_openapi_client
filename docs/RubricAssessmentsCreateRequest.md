# RubricAssessmentsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provisional** | Option<**String**> | (optional) Indicates whether this assessment is provisional, defaults to false. | [optional]
**r#final** | Option<**String**> | (optional) Indicates a provisional grade will be marked as final. It only takes effect if the provisional param is passed as true. Defaults to false. | [optional]
**graded_anonymously** | Option<**bool**> | (optional) Defaults to false | [optional]
**rubric_assessment** | Option<**String**> | A Hash of data to complement the rubric assessment: The user id that refers to the person being assessed   rubric_assessment[user_id] Assessment type. There are only three valid types:  'grading', 'peer_review', or 'provisional_grade'   rubric_assessment[assessment_type] The points awarded for this row.   rubric_assessment[criterion_id][points] Comments to add for this row.   rubric_assessment[criterion_id][comments] For each criterion_id, change the id by the criterion number, ex: criterion_123 If the criterion_id is not specified it defaults to false, and nothing is updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


