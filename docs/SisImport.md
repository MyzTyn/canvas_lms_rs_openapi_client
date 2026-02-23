# SisImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the SIS import. | [optional]
**created_at** | Option<**String**> | The date the SIS import was created. | [optional]
**ended_at** | Option<**String**> | The date the SIS import finished. Returns null if not finished. | [optional]
**updated_at** | Option<**String**> | The date the SIS import was last updated. | [optional]
**workflow_state** | Option<**WorkflowState**> | The current state of the SIS import.  - 'initializing': The SIS import is being created, if this gets stuck in initializing, it will not import and will continue on to next import.  - 'created': The SIS import has been created.  - 'importing': The SIS import is currently processing.  - 'cleanup_batch': The SIS import is currently cleaning up courses, sections, and enrollments not included in the batch for batch_mode imports.  - 'imported': The SIS import has completed successfully.  - 'imported_with_messages': The SIS import completed with errors or warnings.  - 'aborted': The SIS import was aborted.  - 'failed_with_messages': The SIS import failed with errors.  - 'failed': The SIS import failed.  - 'restoring': The SIS import is restoring states of imported items.  - 'partially_restored': The SIS import is restored some of the states of imported items. This is generally due to passing a param like undelete only.  - 'restored': The SIS import is restored all of the states of imported items. (enum: initializing, created, importing, cleanup_batch, imported, imported_with_messages, aborted, failed, failed_with_messages, restoring, partially_restored, restored) | [optional]
**data** | Option<[**models::SisImportData**](SisImportData.md)> |  | [optional]
**statistics** | Option<[**models::SisImportStatistics**](SisImportStatistics.md)> |  | [optional]
**progress** | Option<**String**> | The progress of the SIS import. The progress will reset when using batch_mode and have a different progress for the cleanup stage | [optional]
**errors_attachment** | Option<[**models::File**](File.md)> |  | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**processing_warnings** | Option<[**Vec<Vec<String>>**](Vec.md)> | Only imports that are complete will get this data. An array of CSV_file/warning_message pairs. | [optional]
**processing_errors** | Option<[**Vec<Vec<String>>**](Vec.md)> | An array of CSV_file/error_message pairs. | [optional]
**batch_mode** | Option<**bool**> | Whether the import was run in batch mode. | [optional]
**batch_mode_term_id** | Option<**String**> | The term the batch was limited to. | [optional]
**multi_term_batch_mode** | Option<**bool**> | Enables batch mode against all terms in term file. Requires change_threshold to be set. | [optional]
**skip_deletes** | Option<**bool**> | When set the import will skip any deletes. | [optional]
**override_sis_stickiness** | Option<**bool**> | Whether UI changes were overridden. | [optional]
**add_sis_stickiness** | Option<**bool**> | Whether stickiness was added to the batch changes. | [optional]
**clear_sis_stickiness** | Option<**bool**> | Whether stickiness was cleared. | [optional]
**diffing_threshold_exceeded** | Option<**bool**> | Whether a diffing job failed because the threshold limit got exceeded. | [optional]
**diffing_data_set_identifier** | Option<**String**> | The identifier of the data set that this SIS batch diffs against | [optional]
**diffing_remaster** | Option<**bool**> | Whether diffing remaster data was enabled. | [optional]
**diffed_against_import_id** | Option<**i32**> | The ID of the SIS Import that this import was diffed against | [optional]
**csv_attachments** | Option<[**Vec<Vec<models::File>>**](Vec.md)> | An array of CSV files for processing | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


