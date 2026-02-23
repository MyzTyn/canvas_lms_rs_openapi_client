# WebZipExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the export | [optional]
**created_at** | Option<**String**> | the date and time this export was requested | [optional]
**updated_at** | Option<**String**> | the date and time this export was last updated | [optional]
**zip_attachment** | Option<[**models::File**](File.md)> |  | [optional]
**progress_id** | Option<**i32**> | the unique identifier for the progress object | [optional]
**progress_url** | Option<**String**> | The api endpoint for polling the current progress | [optional]
**user_id** | Option<**i32**> | The ID of the user who started the export | [optional]
**course_id** | Option<**i32**> | The ID of the course the export is for | [optional]
**content_export_id** | Option<**i32**> | The ID of the content export used in the offline export | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the web zip export: created exporting exported generating generated failed (enum: created, exporting, exported, generating, generated, failed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


