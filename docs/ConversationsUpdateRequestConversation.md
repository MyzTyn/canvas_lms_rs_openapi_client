# ConversationsUpdateRequestConversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_state** | Option<**WorkflowState**> | [String, \"read\"|\"unread\"|\"archived\"] Change the state of this conversation (enum: read, unread, archived) | [optional]
**subscribed** | Option<**bool**> | [Boolean] Toggle the current user's subscription to the conversation (only valid for group conversations). If unsubscribed, the user will still have access to the latest messages, but the conversation won't be automatically flagged as unread, nor will it jump to the top of the inbox. | [optional]
**starred** | Option<**bool**> | [Boolean] Toggle the starred state of the current user's view of the conversation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


