# AssignmentOverridesUpdateRequestAssignmentOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**student_ids** | Option<**String**> | [] [Integer] The IDs of the override's target students. If present, the IDs must each identify a user with an active student enrollment in the course that is not already targetted by a different adhoc override. Ignored unless the override being updated is adhoc. | [optional]
**title** | Option<**String**> | [String] The title of an adhoc assignment override. Ignored unless the override being updated is adhoc. | [optional]
**due_at** | Option<**String**> | [DateTime] The day/time the overridden assignment is due. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. If absent, this override will not affect due date. May be present but null to indicate the override removes any previous due date. | [optional]
**unlock_at** | Option<**String**> | [DateTime] The day/time the overridden assignment becomes unlocked. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. If absent, this override will not affect the unlock date. May be present but null to indicate the override removes any previous unlock date. | [optional]
**lock_at** | Option<**String**> | [DateTime] The day/time the overridden assignment becomes locked. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. If absent, this override will not affect the lock date. May be present but null to indicate the override removes any previous lock date. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


