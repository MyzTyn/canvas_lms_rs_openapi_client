# SubmissionsApiUpdateAnonymousForCoursesRequestComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text_comment** | Option<**String**> | [String] Add a textual comment to the submission. | [optional]
**group_comment** | Option<**bool**> | [Boolean] Whether or not this comment should be sent to the entire group (defaults to false). Ignored if this is not a group assignment or if no text_comment is provided. | [optional]
**media_comment_id** | Option<**String**> | [String] Add an audio/video comment to the submission. Media comments can be added via this API, however, note that there is not yet an API to generate or list existing media comments, so this functionality is currently of limited use. | [optional]
**media_comment_type** | Option<**MediaCommentType**> | [String, \"audio\"|\"video\"] The type of media comment being added. (enum: audio, video) | [optional]
**file_ids** | Option<**String**> | [] [Integer] Attach files to this comment that were previously uploaded using the Submission Comment API's files action | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


