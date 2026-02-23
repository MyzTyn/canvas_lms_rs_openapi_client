# AccountNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<**String**> | The subject of the notifications | [optional]
**message** | Option<**String**> | The message to be sent in the notification. | [optional]
**start_at** | Option<**String**> | When to send out the notification. | [optional]
**end_at** | Option<**String**> | When to expire the notification. | [optional]
**icon** | Option<**Icon**> | The icon to display with the message.  Defaults to warning. (enum: warning, information, question, error, calendar) | [optional]
**roles** | Option<**Vec<String>**> | (Deprecated) The roles to send the notification to.  If roles is not passed it defaults to all roles | [optional]
**role_ids** | Option<**Vec<i32>**> | The roles to send the notification to.  If roles is not passed it defaults to all roles | [optional]
**author** | Option<**serde_json::Value**> | The author of the notification. Available only to admins using include_all. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


