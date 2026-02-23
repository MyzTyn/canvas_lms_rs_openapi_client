# GroupCategoriesCreateForAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the group category | 
**non_collaborative** | Option<**bool**> | Can only be set by users with the Differentiation Tag - Add permission  If set to true, groups in this category will be only be visible to users with the Differentiation Tag - Manage permission. | [optional]
**self_signup** | Option<**SelfSignup**> | Allow students to sign up for a group themselves (Course Only). valid values are: \"enabled\":: allows students to self sign up for any group in course \"restricted\":: allows students to self sign up only for groups in the                same section null disallows self sign up (enum: enabled, restricted) | [optional]
**auto_leader** | Option<**AutoLeader**> | Assigns group leaders automatically when generating and allocating students to groups Valid values are: \"first\":: the first student to be allocated to a group is the leader \"random\":: a random student from all members is chosen as the leader (enum: first, random) | [optional]
**group_limit** | Option<**i32**> | Limit the maximum number of users in each group (Course Only). Requires self signup. | [optional]
**sis_group_category_id** | Option<**String**> | The unique SIS identifier. | [optional]
**create_group_count** | Option<**i32**> | Create this number of groups (Course Only). | [optional]
**split_group_count** | Option<[**models::GroupCategoriesCreateForAccountsRequestSplitGroupCount**](GroupCategoriesCreateForAccountsRequestSplitGroupCount.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


