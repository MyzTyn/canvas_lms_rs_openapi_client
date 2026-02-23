# GroupsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the group | [optional]
**description** | Option<**String**> | A description of the group | [optional]
**is_public** | Option<**bool**> | Whether the group is public (applies only to community groups). Currently you cannot set a group back to private once it has been made public. | [optional]
**join_level** | Option<**JoinLevel**> |  (enum: parent_context_auto_join, parent_context_request, invitation_only) | [optional]
**avatar_id** | Option<**i32**> | The id of the attachment previously uploaded to the group that you would like to use as the avatar image for this group. | [optional]
**storage_quota_mb** | Option<**i32**> | The allowed file storage for the group, in megabytes. This parameter is ignored if the caller does not have the manage_storage_quotas permission. | [optional]
**members** | Option<**Vec<String>**> | [String] An array of user ids for users you would like in the group. Users not in the group will be sent invitations. Existing group members who aren't in the list will be removed from the group. | [optional]
**sis_group_id** | Option<**String**> | The sis ID of the group. Must have manage_sis permission to set. | [optional]
**override_sis_stickiness** | Option<**bool**> | Default is true. If false, any fields containing “sticky” changes will not be updated. See SIS CSV Format documentation for information on which fields can have SIS stickiness | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


