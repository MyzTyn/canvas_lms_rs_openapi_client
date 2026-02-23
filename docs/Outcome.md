# Outcome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the outcome | [optional]
**url** | Option<**String**> | the URL for fetching/updating the outcome. should be treated as opaque | [optional]
**context_id** | Option<**i32**> | the context owning the outcome. may be null for global outcomes | [optional]
**context_type** | Option<**String**> |  | [optional]
**title** | Option<**String**> | title of the outcome | [optional]
**display_name** | Option<**String**> | Optional friendly name for reporting | [optional]
**description** | Option<**String**> | description of the outcome. omitted in the abbreviated form. | [optional]
**vendor_guid** | Option<**String**> | A custom GUID for the learning standard. | [optional]
**points_possible** | Option<**i32**> | maximum points possible. included only if the outcome embeds a rubric criterion. omitted in the abbreviated form. | [optional]
**mastery_points** | Option<**i32**> | points necessary to demonstrate mastery outcomes. included only if the outcome embeds a rubric criterion. omitted in the abbreviated form. | [optional]
**calculation_method** | Option<**CalculationMethod**> | the method used to calculate a students score (enum: weighted_average, decaying_average, n_mastery, latest, highest, average) | [optional]
**calculation_int** | Option<**i32**> | this defines the variable value used by the calculation_method. included only if calculation_method uses it | [optional]
**ratings** | Option<[**Vec<models::RubricRating>**](RubricRating.md)> | possible ratings for this outcome. included only if the outcome embeds a rubric criterion. omitted in the abbreviated form. | [optional]
**can_edit** | Option<**bool**> | whether the current user can update the outcome | [optional]
**can_unlink** | Option<**bool**> | whether the outcome can be unlinked | [optional]
**assessed** | Option<**bool**> | whether this outcome has been used to assess a student | [optional]
**has_updateable_rubrics** | Option<**bool**> | whether updates to this outcome will propagate to unassessed rubrics that have imported it | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


