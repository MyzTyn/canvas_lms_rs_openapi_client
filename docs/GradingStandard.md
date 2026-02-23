# GradingStandard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | the title of the grading standard | [optional]
**id** | Option<**i32**> | the id of the grading standard | [optional]
**context_type** | Option<**String**> | the context this standard is associated with, either 'Account' or 'Course' | [optional]
**context_id** | Option<**i32**> | the id for the context either the Account or Course id | [optional]
**points_based** | Option<**bool**> | whether this is a points-based standard | [optional]
**scaling_factor** | Option<**f64**> | the factor by which to scale a score. 1 for percentage based schemss and the max value of points for points based schemes. This number cannot be changed for percentage based schemes. | [optional]
**grading_scheme** | Option<[**Vec<models::GradingSchemeEntry>**](GradingSchemeEntry.md)> | A list of GradingSchemeEntry that make up the Grading Standard as an array of values with the scheme name and value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


