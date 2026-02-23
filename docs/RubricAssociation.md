# RubricAssociation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the association | [optional]
**rubric_id** | Option<**i32**> | the ID of the rubric | [optional]
**association_id** | Option<**i32**> | the ID of the object this association links to | [optional]
**association_type** | Option<**String**> | the type of object this association links to | [optional]
**use_for_grading** | Option<**bool**> | Whether or not the associated rubric is used for grade calculation | [optional]
**summary_data** | Option<**String**> |  | [optional]
**purpose** | Option<**String**> | Whether or not the association is for grading (and thus linked to an assignment) or if it's to indicate the rubric should appear in its context. Values will be grading or bookmark. | [optional]
**hide_score_total** | Option<**bool**> | Whether or not the score total is displayed within the rubric. This option is only available if the rubric is not used for grading. | [optional]
**hide_points** | Option<**bool**> |  | [optional]
**hide_outcome_results** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


