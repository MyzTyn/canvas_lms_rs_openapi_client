# MetadataSaxDocApiUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new display name of the file, with a limit of 255 characters. | [optional]
**parent_folder_id** | Option<**String**> | The id of the folder to move this file into. The new folder must be in the same context as the original parent folder. If the file is in a context without folders this does not apply. | [optional]
**on_duplicate** | Option<**OnDuplicate**> | If the file is moved to a folder containing a file with the same name, or renamed to a name matching an existing file, the API call will fail unless this parameter is supplied.  \"overwrite\":: Replace the existing file with the same name \"rename\":: Add a qualifier to make the new filename unique (enum: overwrite, rename) | [optional]
**lock_at** | Option<**String**> | The datetime to lock the file at | [optional]
**unlock_at** | Option<**String**> | The datetime to unlock the file at | [optional]
**locked** | Option<**bool**> | Flag the file as locked | [optional]
**hidden** | Option<**bool**> | Flag the file as hidden | [optional]
**visibility_level** | Option<**String**> | Configure which roles can access this file | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


