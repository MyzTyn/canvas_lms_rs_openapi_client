# EpubExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the export | [optional]
**created_at** | Option<**String**> | the date and time this export was requested | [optional]
**attachment** | Option<[**models::File**](File.md)> |  | [optional]
**progress_url** | Option<**String**> | The api endpoint for polling the current progress | [optional]
**user_id** | Option<**i32**> | The ID of the user who started the export | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the ePub export: created exporting exported generating generated failed (enum: created, exporting, exported, generating, generated, failed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


