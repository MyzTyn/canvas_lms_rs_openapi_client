# ContentShare

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the content share for the current user | [optional]
**name** | Option<**String**> | The name of the shared content | [optional]
**content_type** | Option<**String**> | The type of content that was shared. Can be assignment, discussion_topic, page, quiz, module, or module_item. | [optional]
**created_at** | Option<**String**> | The datetime the content was shared with this user. | [optional]
**updated_at** | Option<**String**> | The datetime the content was updated. | [optional]
**user_id** | Option<**i32**> | The id of the user who sent or received the content share. | [optional]
**sender** | Option<**serde_json::Value**> | The user who shared the content. This field is provided only to receivers; it is not populated in the sender's list of sent content shares. | [optional]
**receivers** | Option<**Vec<serde_json::Value>**> | An Array of users the content is shared with.  This field is provided only to senders; an empty array will be returned for the receiving users. | [optional]
**source_course** | Option<**serde_json::Value**> | The course the content was originally shared from. | [optional]
**read_state** | Option<**String**> | Whether the recipient has viewed the content share. | [optional]
**content_export** | Option<[**models::ContentExport**](ContentExport.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


