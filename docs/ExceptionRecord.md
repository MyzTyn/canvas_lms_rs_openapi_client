# ExceptionRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**course_id** | Option<**i64**> | The ID of the associated course | [optional]
**conflicting_changes** | Option<**Vec<serde_json::Value>**> | A list of change classes in the associated course's copy of the item that prevented a blueprint change from being applied. One or more of ['content', 'points', 'due_dates', 'availability_dates']. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


