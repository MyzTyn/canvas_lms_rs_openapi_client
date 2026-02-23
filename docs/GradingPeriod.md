# GradingPeriod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier for the grading period. | 
**title** | Option<**String**> | The title for the grading period. | [optional]
**start_date** | **String** | The start date of the grading period. | 
**end_date** | **String** | The end date of the grading period. | 
**close_date** | Option<**String**> | Grades can only be changed before the close date of the grading period. | [optional]
**weight** | Option<**i32**> | A weight value that contributes to the overall weight of a grading period set which is used to calculate how much assignments in this period contribute to the total grade | [optional]
**is_closed** | Option<**bool**> | If true, the grading period's close_date has passed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


