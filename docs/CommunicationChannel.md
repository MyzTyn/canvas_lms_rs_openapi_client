# CommunicationChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the communication channel. | [optional]
**address** | Option<**String**> | The address, or path, of the communication channel. | [optional]
**r#type** | Option<**Type**> | The type of communcation channel being described. Possible values are: 'email', 'push', 'sms'. This field determines the type of value seen in 'address'. (enum: email, push, sms) | [optional]
**position** | Option<**i32**> | The position of this communication channel relative to the user's other channels when they are ordered. | [optional]
**user_id** | Option<**i32**> | The ID of the user that owns this communication channel. | [optional]
**bounce_count** | Option<**i32**> | The number of bounces the channel has experienced. This is reset if the channel sends successfully. | [optional]
**last_bounce_at** | Option<**String**> | The time the last bounce occurred. | [optional]
**workflow_state** | Option<**WorkflowState**> | The current state of the communication channel. Possible values are: 'unconfirmed' or 'active'. (enum: unconfirmed, active) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


