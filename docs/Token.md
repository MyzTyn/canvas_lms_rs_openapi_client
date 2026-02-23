# Token

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The internal database ID of the token. | [optional]
**created_at** | Option<**String**> | The time the token was created. | [optional]
**expires_at** | Option<**String**> | The time the token will permanently expire, or null if it does not permanently expire. | [optional]
**workflow_state** | Option<**String**> | The current state of the token. One of 'active', 'pending', 'disabled', or 'deleted'. | [optional]
**remember_access** | Option<**bool**> | Whether the token should be remembered across sessions. Only applicable for OAuth tokens. | [optional]
**scopes** | Option<**Vec<String>**> | The scopes associated with the token. If empty, there are no scope limitations. | [optional]
**real_user_id** | Option<**i32**> | If the token was created while masquerading, this is the ID of the real user. Otherwise, null. | [optional]
**token** | Option<**String**> | The actual access token. Only included when the token is first created. | [optional]
**token_hint** | Option<**String**> | A short, unique string that can be used to look up the token. | [optional]
**user_id** | Option<**i32**> | The ID of the user the token belongs to. | [optional]
**purpose** | Option<**String**> | The purpose of the token. | [optional]
**app_name** | Option<**String**> | If the token was created by an OAuth application, this is the name of that application. Otherwise, null. | [optional]
**can_manually_regenerate** | Option<**bool**> | Whether the current user can manually regenerate this token. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


