# AuthenticationEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | timestamp of the event | [optional]
**event_type** | Option<**EventType**> | authentication event type ('login' or 'logout') (enum: login, logout) | [optional]
**pseudonym_id** | Option<**i32**> | ID of the pseudonym (login) associated with the event | [optional]
**account_id** | Option<**i32**> | ID of the account associated with the event. will match the account_id in the associated pseudonym. | [optional]
**user_id** | Option<**i32**> | ID of the user associated with the event will match the user_id in the associated pseudonym. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


