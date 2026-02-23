# GroupsCreateOtherRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the group | [optional]
**description** | Option<**String**> | A description of the group | [optional]
**is_public** | Option<**bool**> | whether the group is public (applies only to community groups) | [optional]
**join_level** | Option<**JoinLevel**> |  (enum: parent_context_auto_join, parent_context_request, invitation_only) | [optional]
**storage_quota_mb** | Option<**i32**> | The allowed file storage for the group, in megabytes. This parameter is ignored if the caller does not have the manage_storage_quotas permission. | [optional]
**sis_group_id** | Option<**String**> | The sis ID of the group. Must have manage_sis permission to set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


