# AsyncQueryStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_id** | **uuid::Uuid** | The UUID of the query being polled | 
**status** | **String** | Current processing status of the query | 
**format** | **String** | The format that results will be returned in | 
**results_url** | Option<**String**> | URL to retrieve query results. Only present when status is 'finished' | [optional]
**error_code** | Option<**String**> | Error code indicating the reason for query failure, if applicable | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


