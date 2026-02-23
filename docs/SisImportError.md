# SisImportError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sis_import_id** | Option<**i32**> | The unique identifier for the SIS import. | [optional]
**file** | Option<**String**> | The file where the error message occurred. | [optional]
**message** | Option<**String**> | The error message that from the record. | [optional]
**row_info** | Option<**String**> | The contents of the line that had the error. | [optional]
**row** | Option<**i32**> | The line number where the error occurred. Some Importers do not yet support this. This is a 1 based index starting with the header row. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


