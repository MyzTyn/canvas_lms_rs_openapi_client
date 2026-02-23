# LtiContextControl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the Canvas ID of the Lti_ContextControl object | [optional]
**course_id** | Option<**i32**> | the Canvas ID of the Course that owns this. one of this or account_id will always be present | [optional]
**account_id** | Option<**i32**> | the Canvas ID of the Account that owns this. one of this or course_id will always be present | [optional]
**deployment_id** | Option<**i32**> | the Canvas ID of the ContextExternalTool that owns this, representing an LTI deployment | [optional]
**available** | Option<**bool**> | The state of this tool in this context. `true` means the tool is available in this context and in all contexts below it. | [optional]
**path** | Option<**String**> | A representation of the account hierarchy for the context that owns this object. Used for checking availability during LTI operations. | [optional]
**display_path** | Option<**Vec<String>**> | For UI display. Names of the accounts in the context's hierarchy. Excludes the root, and the current account if context is an account. | [optional]
**context_name** | Option<**String**> | For UI display. The name of the context this object is associated with | [optional]
**depth** | Option<**i32**> | For UI display. The depth of ContextControls for this particular deployment account chain, which can be different from the number of accounts in the chain. | [optional]
**course_count** | Option<**i32**> | For UI display. The number of courses in this account and all nested subaccounts. 0 when context is a Course. | [optional]
**child_control_count** | Option<**i32**> | For UI display. The number of controls for accounts below this one, including all nested subaccounts. 0 when context is a Course. | [optional]
**subaccount_count** | Option<**i32**> | For UI display. The number of subaccounts for this account. Includes all nested subaccounts. 0 when context is a Course. | [optional]
**workflow_state** | Option<**String**> | The state of the object | [optional]
**created_at** | Option<**String**> | Timestamp of the object's creation | [optional]
**updated_at** | Option<**String**> | Timestamp of the object's last update | [optional]
**created_by** | Option<[**models::User**](User.md)> |  | [optional]
**updated_by** | Option<[**models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


