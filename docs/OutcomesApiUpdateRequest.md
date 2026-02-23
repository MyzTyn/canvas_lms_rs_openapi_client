# OutcomesApiUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The new outcome title. | [optional]
**display_name** | Option<**String**> | A friendly name shown in reports for outcomes with cryptic titles, such as common core standards names. | [optional]
**description** | Option<**String**> | The new outcome description. | [optional]
**vendor_guid** | Option<**String**> | A custom GUID for the learning standard. | [optional]
**mastery_points** | Option<**i32**> | The new mastery threshold for the embedded rubric criterion. | [optional]
**ratings_left_square_bracket_right_square_bracket_left_square_bracket_description_right_square_bracket** | Option<**String**> | [String] The description of a new rating level for the embedded rubric criterion. | [optional]
**ratings_left_square_bracket_right_square_bracket_left_square_bracket_points_right_square_bracket** | Option<**String**> | [Integer] The points corresponding to a new rating level for the embedded rubric criterion. | [optional]
**calculation_method** | Option<**CalculationMethod**> | The new calculation method. If the Outcomes New Decaying Average Calculation Method FF is ENABLED then \"weighted_average\" can be used and it is same as previous \"decaying_average\" and new \"decaying_average\" will have improved version of calculation. (enum: weighted_average, decaying_average, n_mastery, latest, highest, average) | [optional]
**calculation_int** | Option<**i32**> | The new calculation int.  Only applies if the calculation_method is \"decaying_average\" or \"n_mastery\" | [optional]
**add_defaults** | Option<**bool**> | If defaults are requested, then color and mastery level defaults will be added to outcome ratings in the result. This will only take effect if the Account Level Mastery Scales FF is DISABLED | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


