# CompletionRequirement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**Type**> | one of 'must_view', 'must_submit', 'must_contribute', 'min_score', 'min_percentage', 'must_mark_done' (enum: must_view, must_submit, must_contribute, min_score, min_percentage, must_mark_done) | [optional]
**min_score** | Option<**i32**> | minimum score required to complete (only present when type == 'min_score') | [optional]
**min_percentage** | Option<**i32**> | minimum percentage required to complete (only present when type == 'min_percentage') | [optional]
**completed** | Option<**bool**> | whether the calling user has met this requirement (Optional; present only if the caller is a student or if the optional parameter 'student_id' is included) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


