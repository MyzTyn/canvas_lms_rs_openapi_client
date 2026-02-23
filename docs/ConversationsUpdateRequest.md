# ConversationsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | Option<**Scope**> | Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} (enum: unread, starred, archived) | [optional]
**filter** | Option<**Vec<String>**> | [String, course_id|group_id|user_id] Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} | [optional]
**conversation** | Option<[**models::ConversationsUpdateRequestConversation**](ConversationsUpdateRequestConversation.md)> |  | [optional]
**filter_mode** | Option<[**models::ConversationsCreateRequestFilterMode**](ConversationsCreateRequestFilterMode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


