# GradingPeriodsBatchUpdateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**set_id** | **String** | The id of the grading period set. | 
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_id_right_square_bracket** | Option<**String**> | [String] The id of the grading period. If the id parameter does not exist, a new grading period will be created. | [optional]
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_title_right_square_bracket** | **String** | [Required, String] The title of the grading period. The title is required for creating a new grading period, but not for updating an existing grading period. | 
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_start_date_right_square_bracket** | **String** | [Required, Date] The date the grading period starts. The start_date is required for creating a new grading period, but not for updating an existing grading period. | 
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_end_date_right_square_bracket** | **String** | [Required, Date] The date the grading period ends. The end_date is required for creating a new grading period, but not for updating an existing grading period. | 
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_close_date_right_square_bracket** | **String** | [Required, Date] The date after which grades can no longer be changed for a grading period. The close_date is required for creating a new grading period, but not for updating an existing grading period. | 
**grading_periods_left_square_bracket_right_square_bracket_left_square_bracket_weight_right_square_bracket** | Option<**String**> | [Float] A weight value that contributes to the overall weight of a grading period set which is used to calculate how much assignments in this period contribute to the total grade | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


