# ChangeRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | Option<**i64**> | The ID of the learning object that was changed in the blueprint course. | [optional]
**asset_type** | Option<**String**> | The type of the learning object that was changed in the blueprint course.  One of 'assignment', 'attachment', 'discussion_topic', 'external_tool', 'quiz', 'wiki_page', 'syllabus', or 'settings'.  For 'syllabus' or 'settings', the asset_id is the course id. | [optional]
**asset_name** | Option<**String**> | The name of the learning object that was changed in the blueprint course. | [optional]
**change_type** | Option<**String**> | The type of change; one of 'created', 'updated', 'deleted' | [optional]
**html_url** | Option<**String**> | The URL of the changed object | [optional]
**locked** | Option<**bool**> | Whether the object is locked in the blueprint | [optional]
**exceptions** | Option<**Vec<serde_json::Value>**> | A list of ExceptionRecords for linked courses that did not receive this update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


