# RubricsUpdateRequestRubricAssociation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**association_id** | Option<**i32**> | [Integer] The id of the object with which this rubric is associated | [optional]
**association_type** | Option<**String**> | [\"Assignment\"|\"Course\"|\"Account\"] The type of object this rubric is associated with | [optional]
**use_for_grading** | Option<**bool**> | [Boolean] Whether or not the associated rubric is used for grade calculation | [optional]
**hide_score_total** | Option<**bool**> | [Boolean] Whether or not the score total is displayed within the rubric. This option is only available if the rubric is not used for grading. | [optional]
**purpose** | Option<**String**> | [\"grading\"|\"bookmark\"] Whether or not the association is for grading (and thus linked to an assignment) or if it's to indicate the rubric should appear in its context | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


