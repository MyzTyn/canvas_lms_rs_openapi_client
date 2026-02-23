# CommMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the CommMessage. | [optional]
**created_at** | Option<**String**> | The date and time this message was created | [optional]
**sent_at** | Option<**String**> | The date and time this message was sent | [optional]
**workflow_state** | Option<**WorkflowState**> | The workflow state of the message. Possible values: 'created' : The message has been created, but not yet processed. 'staged' : The message is queued for sending. 'sending' : The message is being sent currently. 'sent' : The message has been successfully sent. 'bounced' : An error occurred during the sending of the message.'dashboard' : The message has been sent to the dashboard. 'closed' :  The message has been sent and closed, typically for dashboard messages or messages sent to deleted users. 'cancelled' : The message was cancelled before it could be sent. (enum: created, staged, sending, sent, bounced, dashboard, cancelled, closed) | [optional]
**from** | Option<**String**> | The address that was put in the 'from' field of the message | [optional]
**from_name** | Option<**String**> | The display name for the from address | [optional]
**to** | Option<**String**> | The address the message was sent to: | [optional]
**reply_to** | Option<**String**> | The reply_to header of the message | [optional]
**subject** | Option<**String**> | The message subject | [optional]
**body** | Option<**String**> | The plain text body of the message | [optional]
**html_body** | Option<**String**> | The HTML body of the message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


