# GroupMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the membership object | [optional]
**group_id** | Option<**i32**> | The id of the group object to which the membership belongs | [optional]
**user_id** | Option<**i32**> | The id of the user object to which the membership belongs | [optional]
**workflow_state** | Option<**WorkflowState**> | The current state of the membership. Current possible values are 'accepted', 'invited', and 'requested' (enum: accepted, invited, requested) | [optional]
**moderator** | Option<**bool**> | Whether or not the user is a moderator of the group (the must also be an active member of the group to moderate) | [optional]
**just_created** | Option<**bool**> | optional: whether or not the record was just created on a create call (POST), i.e. was the user just added to the group, or was the user already a member | [optional]
**sis_import_id** | Option<**i32**> | The id of the SIS import if created through SIS. Only included if the user has permission to manage SIS information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


