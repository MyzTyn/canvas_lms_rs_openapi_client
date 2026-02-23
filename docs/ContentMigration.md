# ContentMigration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the migration | [optional]
**migration_type** | Option<**String**> | the type of content migration | [optional]
**migration_type_title** | Option<**String**> | the name of the content migration type | [optional]
**migration_issues_url** | Option<**String**> | API url to the content migration's issues | [optional]
**attachment** | Option<**String**> | attachment api object for the uploaded file may not be present for all migrations | [optional]
**progress_url** | Option<**String**> | The api endpoint for polling the current progress | [optional]
**user_id** | Option<**i32**> | The user who started the migration | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the content migration: pre_processing, pre_processed, running, waiting_for_select, completed, failed (enum: pre_processing, pre_processed, running, waiting_for_select, completed, failed) | [optional]
**started_at** | Option<**String**> | timestamp | [optional]
**finished_at** | Option<**String**> | timestamp | [optional]
**pre_attachment** | Option<**String**> | file uploading data, see {file:file.file_uploads.html File Upload Documentation} for file upload workflow This works a little differently in that all the file data is in the pre_attachment hash if there is no upload_url then there was an attachment pre-processing error, the error message will be in the message key This data will only be here after a create or update call | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


