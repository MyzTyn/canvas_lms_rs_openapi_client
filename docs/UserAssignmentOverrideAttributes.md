# UserAssignmentOverrideAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique Canvas identifier for the assignment override | [optional]
**title** | Option<**String**> | The title of the assignment override. | [optional]
**due_at** | Option<**String**> | The time at which this assignment is due | [optional]
**unlock_at** | Option<**String**> | (Optional) Time at which this was/will be unlocked. | [optional]
**lock_at** | Option<**String**> | (Optional) Time at which this was/will be locked. | [optional]
**students** | Option<[**Vec<models::StudentAttributes>**](StudentAttributes.md)> | Includes attributes of a student for convenience. For more details see Users API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


