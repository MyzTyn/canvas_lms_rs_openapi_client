# FoldersUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new name of the folder | [optional]
**parent_folder_id** | Option<**String**> | The id of the folder to move this folder into. The new folder must be in the same context as the original parent folder. | [optional]
**lock_at** | Option<**String**> | The datetime to lock the folder at | [optional]
**unlock_at** | Option<**String**> | The datetime to unlock the folder at | [optional]
**locked** | Option<**bool**> | Flag the folder as locked | [optional]
**hidden** | Option<**bool**> | Flag the folder as hidden | [optional]
**position** | Option<**i32**> | Set an explicit sort position for the folder | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


