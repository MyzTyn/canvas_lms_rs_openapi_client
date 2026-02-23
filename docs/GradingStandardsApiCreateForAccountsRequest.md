# GradingStandardsApiCreateForAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title for the Grading Standard. | 
**points_based** | Option<**bool**> | Whether or not a grading scheme is points based. Defaults to false. | [optional]
**scaling_factor** | **i32** | The factor by which to scale a percentage into a points based scheme grade. This is the maximum number of points possible in the grading scheme. Defaults to 1. Not required for percentage based grading schemes. | 
**grading_scheme_entry_left_square_bracket_right_square_bracket_left_square_bracket_name_right_square_bracket** | **String** | [Required, String] The name for an entry value within a GradingStandard that describes the range of the value e.g. A- | 
**grading_scheme_entry_left_square_bracket_right_square_bracket_left_square_bracket_value_right_square_bracket** | **String** | [Required, Integer] The value for the name of the entry within a GradingStandard. The entry represents the lower bound of the range for the entry. This range includes the value up to the next entry in the GradingStandard, or 100 if there is no upper bound. The lowest value will have a lower bound range of 0. e.g. 93 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


