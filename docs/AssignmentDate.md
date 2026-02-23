# AssignmentDate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | (Optional, missing if 'base' is present) id of the assignment override this date represents | [optional]
**base** | Option<**bool**> | (Optional, present if 'id' is missing) whether this date represents the assignment's or quiz's default due date | [optional]
**title** | Option<**String**> |  | [optional]
**due_at** | Option<**String**> | The due date for the assignment. Must be between the unlock date and the lock date if there are lock dates | [optional]
**unlock_at** | Option<**String**> | The unlock date for the assignment. Must be before the due date if there is a due date. | [optional]
**lock_at** | Option<**String**> | The lock date for the assignment. Must be after the due date if there is a due date. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


