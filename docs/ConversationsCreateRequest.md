# ConversationsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipients** | **Vec<String>** | [Required, String] An array of recipient ids. These may be user ids | 
**subject** | Option<**String**> | The subject of the conversation. This is ignored when reusing a conversation. Maximum length is 255 characters. | [optional]
**body** | **String** | The message to be sent | 
**force_new** | Option<**bool**> | Forces a new message to be created, even if there is an existing private conversation. | [optional]
**group_conversation** | Option<**bool**> | Defaults to false.  When false, individual private conversations will be created with each recipient. If true, this will be a group conversation (i.e. all recipients may see all messages and replies). Must be set true if the number of recipients is over the set maximum (default is 100). | [optional]
**attachment_ids** | Option<**Vec<String>**> | [String] An array of attachments ids. These must be files that have been previously uploaded to the sender's \"conversation attachments\" folder. | [optional]
**media_comment_id** | Option<**String**> | Media comment id of an audio or video file to be associated with this message. | [optional]
**media_comment_type** | Option<**MediaCommentType**> | Type of the associated media file (enum: audio, video) | [optional]
**mode** | Option<**Mode**> | Determines whether the messages will be created/sent synchronously or asynchronously. Defaults to sync, and this option is ignored if this is a group conversation or there is just one recipient (i.e. it must be a bulk private message). When sent async, the response will be an empty array (batch status can be queried via the {api:ConversationsController#batches batches API}) (enum: sync, async) | [optional]
**scope** | Option<**Scope**> | Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} (enum: unread, starred, archived) | [optional]
**filter** | Option<**Vec<String>**> | [String, course_id|group_id|user_id] Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} | [optional]
**context_code** | Option<**String**> | The course or group that is the context for this conversation. Same format as courses or groups in the recipients argument. | [optional]
**include** | Option<**Vec<String>**> | [Optional, String, \"uuid\"] \"uuid\":: Optionally include an \"uuid\" key for each user participating in the conversation | [optional]
**filter_mode** | Option<[**models::ConversationsCreateRequestFilterMode**](ConversationsCreateRequestFilterMode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


