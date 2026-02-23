# GradingSchemeEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for an entry value within a GradingStandard that describes the range of the value | [optional]
**value** | Option<**i32**> | The value for the name of the entry within a GradingStandard. The entry represents the lower bound of the range for the entry. This range includes the value up to the next entry in the GradingStandard, or the maximum value for the scheme if there is no upper bound. The lowest value will have a lower bound range of 0. | [optional]
**calculated_value** | Option<**i32**> | The value that will be used to compare against a grade. For percentage based grading schemes, this is a number from 0 - 100 representing a percent. For point based grading schemes, this is the lower bound of points to achieve the grade. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


