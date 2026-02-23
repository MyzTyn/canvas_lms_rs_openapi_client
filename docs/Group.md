# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the group. | [optional]
**name** | Option<**String**> | The display name of the group. | [optional]
**description** | Option<**String**> | A description of the group. This is plain text. | [optional]
**is_public** | Option<**bool**> | Whether or not the group is public.  Currently only community groups can be made public.  Also, once a group has been set to public, it cannot be changed back to private. | [optional]
**followed_by_user** | Option<**bool**> | Whether or not the current user is following this group. | [optional]
**join_level** | Option<**JoinLevel**> | How people are allowed to join the group.  For all groups except for community groups, the user must share the group's parent course or account.  For student organized or community groups, where a user can be a member of as many or few as they want, the applicable levels are 'parent_context_auto_join', 'parent_context_request', and 'invitation_only'.  For class groups, where students are divided up and should only be part of one group of the category, this value will always be 'invitation_only', and is not relevant. * If 'parent_context_auto_join', anyone can join and will be automatically accepted. * If 'parent_context_request', anyone  can request to join, which must be approved by a group moderator. * If 'invitation_only', only those how have received an invitation my join the group, by accepting that invitation. (enum: parent_context_auto_join, parent_context_request, invitation_only) | [optional]
**members_count** | Option<**i32**> | The number of members currently in the group | [optional]
**avatar_url** | Option<**String**> | The url of the group's avatar | [optional]
**context_type** | Option<**String**> | The course or account that the group belongs to. The pattern here is that whatever the context_type is, there will be an _id field named after that type. So if instead context_type was 'account', the course_id field would be replaced by an account_id field. | [optional]
**context_name** | Option<**String**> | The course or account name that the group belongs to. | [optional]
**course_id** | Option<**i32**> |  | [optional]
**role** | Option<**Role**> | Certain types of groups have special role designations. Currently, these include: 'communities', 'student_organized', and 'imported'. Regular course/account groups have a role of null. (enum: communities, student_organized, imported) | [optional]
**group_category_id** | Option<**i32**> | The ID of the group's category. | [optional]
**sis_group_id** | Option<**String**> | The SIS ID of the group. Only included if the user has permission to view SIS information. | [optional]
**sis_import_id** | Option<**i32**> | The id of the SIS import if created through SIS. Only included if the user has permission to manage SIS information. | [optional]
**storage_quota_mb** | Option<**i32**> | the storage quota for the group, in megabytes | [optional]
**permissions** | Option<**serde_json::Value**> | optional: the permissions the user has for the group. returned only for a single group and include[]=permissions | [optional]
**users** | Option<[**Vec<models::User>**](User.md)> | optional: A list of users that are members in the group. Returned only if include[]=users. WARNING: this collection's size is capped (if there are an extremely large number of users in the group (thousands) not all of them will be returned). If you need to capture all the users in a group with certainty or experiencing slow response consider using the paginated /api/v1/groups/<group_id>/users endpoint. | [optional]
**non_collaborative** | Option<**bool**> | Indicates whether this group category is non-collaborative. A value of true means these group categories rely on the manage_tags permissions and do not have collaborative features | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


