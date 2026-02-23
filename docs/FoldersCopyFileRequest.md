# FoldersCopyFileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_file_id** | **String** | The id of the source file | 
**on_duplicate** | Option<**OnDuplicate**> | What to do if a file with the same name already exists at the destination. If such a file exists and this parameter is not given, the call will fail.  \"overwrite\":: Replace an existing file with the same name \"rename\":: Add a qualifier to make the new filename unique (enum: overwrite, rename) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


