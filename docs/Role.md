# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the role | [optional]
**label** | Option<**String**> | The label of the role. | [optional]
**role** | Option<**String**> | The label of the role. (Deprecated alias for 'label') | [optional]
**base_role_type** | Option<**String**> | The role type that is being used as a base for this role. For account-level roles, this is 'AccountMembership'. For course-level roles, it is an enrollment type. | [optional]
**is_account_role** | Option<**bool**> | Whether this role applies to account memberships (i.e., not linked to an enrollment in a course). | [optional]
**account** | Option<[**models::Account**](Account.md)> |  | [optional]
**workflow_state** | Option<**String**> | The state of the role: 'active', 'inactive', or 'built_in' | [optional]
**created_at** | Option<**String**> | The date and time the role was created. | [optional]
**last_updated_at** | Option<**String**> | The date and time the role was last updated. | [optional]
**permissions** | Option<**serde_json::Value**> | A dictionary of permissions keyed by name (see 'List assignable permissions' API). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


