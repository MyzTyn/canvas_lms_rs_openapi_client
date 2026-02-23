# GroupCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the group category. | [optional]
**name** | Option<**String**> | The display name of the group category. | [optional]
**role** | Option<**String**> | Certain types of group categories have special role designations. Currently, these include: 'communities', 'student_organized', and 'imported'. Regular course/account group categories have a role of null. | [optional]
**self_signup** | Option<**SelfSignup**> | If the group category allows users to join a group themselves, thought they may only be a member of one group per group category at a time. Values include 'restricted', 'enabled', and null 'enabled' allows students to assign themselves to a group 'restricted' restricts them to only joining a group in their section null disallows students from joining groups (enum: restricted, enabled) | [optional]
**auto_leader** | Option<**AutoLeader**> | Gives instructors the ability to automatically have group leaders assigned.  Values include 'random', 'first', and null; 'random' picks a student from the group at random as the leader, 'first' sets the first student to be assigned to the group as the leader (enum: first, random) | [optional]
**context_type** | Option<**String**> | The course or account that the category group belongs to. The pattern here is that whatever the context_type is, there will be an _id field named after that type. So if instead context_type was 'Course', the course_id field would be replaced by an course_id field. | [optional]
**account_id** | Option<**i32**> |  | [optional]
**group_limit** | Option<**i32**> | If self-signup is enabled, group_limit can be set to cap the number of users in each group. If null, there is no limit. | [optional]
**sis_group_category_id** | Option<**String**> | The SIS identifier for the group category. This field is only included if the user has permission to manage or view SIS information. | [optional]
**sis_import_id** | Option<**i32**> | The unique identifier for the SIS import. This field is only included if the user has permission to manage SIS information. | [optional]
**progress** | Option<[**models::Progress**](Progress.md)> |  | [optional]
**non_collaborative** | Option<**bool**> | Indicates whether this group category is non-collaborative. A value of true means these group categories rely on the manage_tags permissions and do not have collaborative features | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


