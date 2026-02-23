# Conversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | the unique identifier for the conversation. | [optional]
**subject** | Option<**String**> | the subject of the conversation. | [optional]
**workflow_state** | Option<**String**> | The current state of the conversation (read, unread or archived). | [optional]
**last_message** | Option<**String**> | A <=100 character preview from the most recent message. | [optional]
**start_at** | Option<**String**> | the date and time at which the last message was sent. | [optional]
**message_count** | Option<**i32**> | the number of messages in the conversation. | [optional]
**subscribed** | Option<**bool**> | whether the current user is subscribed to the conversation. | [optional]
**private** | Option<**bool**> | whether the conversation is private. | [optional]
**starred** | Option<**bool**> | whether the conversation is starred. | [optional]
**properties** | Option<**Vec<String>**> | Additional conversation flags (last_author, attachments, media_objects). Each listed property means the flag is set to true (i.e. the current user is the most recent author, there are attachments, or there are media objects) | [optional]
**audience** | Option<**Vec<i32>**> | Array of user ids who are involved in the conversation, ordered by participation level, then alphabetical. Excludes current user, unless this is a monologue. | [optional]
**audience_contexts** | Option<**Vec<String>**> | Most relevant shared contexts (courses and groups) between current user and other participants. If there is only one participant, it will also include that user's enrollment(s)/ membership type(s) in each course/group. | [optional]
**avatar_url** | Option<**String**> | URL to appropriate icon for this conversation (custom, individual or group avatar, depending on audience). | [optional]
**participants** | Option<[**Vec<models::ConversationParticipant>**](ConversationParticipant.md)> | Array of users participating in the conversation. Includes current user. | [optional]
**visible** | Option<**bool**> | indicates whether the conversation is visible under the current scope and filter. This attribute is always true in the index API response, and is primarily useful in create/update responses so that you can know if the record should be displayed in the UI. The default scope is assumed, unless a scope or filter is passed to the create/update API call. | [optional]
**context_name** | Option<**String**> | Name of the course or group in which the conversation is occurring. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


