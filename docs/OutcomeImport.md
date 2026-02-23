# OutcomeImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the outcome import. | [optional]
**learning_outcome_group_id** | Option<**i32**> | The unique identifier for the group into which the outcomes will be imported to, or NULL. | [optional]
**created_at** | Option<**String**> | The date the outcome import was created. | [optional]
**ended_at** | Option<**String**> | The date the outcome import finished. Returns null if not finished. | [optional]
**updated_at** | Option<**String**> | The date the outcome import was last updated. | [optional]
**workflow_state** | Option<**WorkflowState**> | The current state of the outcome import.  - 'created': The outcome import has been created.  - 'importing': The outcome import is currently processing.  - 'succeeded': The outcome import has completed successfully.  - 'failed': The outcome import failed. (enum: created, importing, succeeded, failed) | [optional]
**data** | Option<[**models::OutcomeImportData**](OutcomeImportData.md)> |  | [optional]
**progress** | Option<**String**> | The progress of the outcome import. | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**processing_errors** | Option<[**Vec<Vec<serde_json::Value>>**](Vec.md)> | An array of row number / error message pairs. Returns the first 25 errors. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


