# ConversationsAddMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | The message to be sent. | 
**attachment_ids** | Option<**Vec<String>**> | [String] An array of attachments ids. These must be files that have been previously uploaded to the sender's \"conversation attachments\" folder. | [optional]
**media_comment_id** | Option<**String**> | Media comment id of an audio of video file to be associated with this message. | [optional]
**media_comment_type** | Option<**MediaCommentType**> | Type of the associated media file. (enum: audio, video) | [optional]
**recipients** | Option<**Vec<String>**> | [String] | [optional]
**included_messages** | Option<**Vec<String>**> | [String] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


