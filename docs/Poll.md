# Poll

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier for the poll. | 
**question** | **String** | The question/title of the poll. | 
**description** | Option<**String**> | A short description of the poll. | [optional]
**created_at** | Option<**String**> | The time at which the poll was created. | [optional]
**user_id** | Option<**i32**> | The unique identifier for the user that created the poll. | [optional]
**total_results** | Option<**serde_json::Value**> | An aggregate of the results of all associated poll sessions, with the poll choice id as the key, and the aggregated submission count as the value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


