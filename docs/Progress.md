# Progress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the Progress object | [optional]
**context_id** | Option<**i32**> | the context owning the job. | [optional]
**context_type** | Option<**String**> |  | [optional]
**user_id** | Option<**i32**> | the id of the user who started the job | [optional]
**tag** | Option<**String**> | the type of operation | [optional]
**completion** | Option<**i32**> | percent completed | [optional]
**workflow_state** | Option<**WorkflowState**> | the state of the job one of 'queued', 'running', 'completed', 'failed' (enum: queued, running, completed, failed) | [optional]
**created_at** | Option<**String**> | the time the job was created | [optional]
**updated_at** | Option<**String**> | the time the job was last updated | [optional]
**message** | Option<**String**> | optional details about the job | [optional]
**results** | Option<**serde_json::Value**> | optional results of the job. omitted when job is still pending | [optional]
**url** | Option<**String**> | url where a progress update can be retrieved | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


