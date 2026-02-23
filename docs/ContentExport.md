# ContentExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the export | [optional]
**created_at** | Option<**String**> | the date and time this export was requested | [optional]
**export_type** | Option<**ExportType**> | the type of content migration: 'common_cartridge' or 'qti' (enum: common_cartridge, qti) | [optional]
**attachment** | Option<[**models::File**](File.md)> |  | [optional]
**progress_url** | Option<**String**> | The api endpoint for polling the current progress | [optional]
**user_id** | Option<**i32**> | The ID of the user who started the export | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the content migration: created exporting exported failed (enum: created, exporting, exported, failed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


