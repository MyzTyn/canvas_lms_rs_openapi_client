# OutcomeGroupsApiLinkOtherRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**outcome_id** | Option<**i32**> | The ID of the existing outcome to link. | [optional]
**move_from** | Option<**i32**> | The ID of the old outcome group. Only used if outcome_id is present. | [optional]
**title** | **String** | The title of the new outcome. Required if outcome_id is absent. | 
**display_name** | Option<**String**> | A friendly name shown in reports for outcomes with cryptic titles, such as common core standards names. | [optional]
**description** | Option<**String**> | The description of the new outcome. | [optional]
**vendor_guid** | Option<**String**> | A custom GUID for the learning standard. | [optional]
**mastery_points** | Option<**i32**> | The mastery threshold for the embedded rubric criterion. | [optional]
**ratings_left_square_bracket_right_square_bracket_left_square_bracket_description_right_square_bracket** | Option<**String**> | [String] The description of a rating level for the embedded rubric criterion. | [optional]
**ratings_left_square_bracket_right_square_bracket_left_square_bracket_points_right_square_bracket** | Option<**String**> | [Integer] The points corresponding to a rating level for the embedded rubric criterion. | [optional]
**calculation_method** | Option<**CalculationMethod**> | The new calculation method.  Defaults to \"decaying_average\" if the Outcomes New Decaying Average Calculation Method FF is ENABLED then Defaults to \"weighted_average\" (enum: weighted_average, decaying_average, n_mastery, latest, highest, average) | [optional]
**calculation_int** | Option<**i32**> | The new calculation int.  Only applies if the calculation_method is \"weighted_average\", \"decaying_average\" or \"n_mastery\". Defaults to 65 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


