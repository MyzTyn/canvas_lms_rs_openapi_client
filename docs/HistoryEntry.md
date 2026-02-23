# HistoryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_code** | **String** | The asset string for the item viewed | 
**asset_name** | **String** | The name of the item | 
**asset_icon** | Option<**String**> | The icon type shown for the item. One of 'icon-announcement', 'icon-assignment', 'icon-calendar-month', 'icon-discussion', 'icon-document', 'icon-download', 'icon-gradebook', 'icon-home', 'icon-message', 'icon-module', 'icon-outcomes', 'icon-quiz', 'icon-user', 'icon-syllabus' | [optional]
**asset_readable_category** | Option<**String**> | The associated category describing the asset_icon | [optional]
**context_type** | Option<**String**> | The type of context of the item visited. One of 'Course', 'Group', 'User', or 'Account' | [optional]
**context_id** | Option<**i64**> | The id of the context, if applicable | [optional]
**context_name** | Option<**String**> | The name of the context | [optional]
**visited_url** | **String** | The URL of the item | 
**visited_at** | **String** | When the page was visited | 
**interaction_seconds** | Option<**i64**> | The estimated time spent on the page in seconds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


