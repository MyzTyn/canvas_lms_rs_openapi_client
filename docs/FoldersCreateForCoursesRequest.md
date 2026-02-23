# FoldersCreateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the folder | 
**parent_folder_id** | Option<**String**> | The id of the folder to store the new folder in. An error will be returned if this does not correspond to an existing folder. If this and parent_folder_path are sent an error will be returned. If neither is given, a default folder will be used. | [optional]
**parent_folder_path** | Option<**String**> | The path of the folder to store the new folder in. The path separator is the forward slash `/`, never a back slash. The parent folder will be created if it does not already exist. This parameter only applies to new folders in a context that has folders, such as a user, a course, or a group. If this and parent_folder_id are sent an error will be returned. If neither is given, a default folder will be used. | [optional]
**lock_at** | Option<**String**> | The datetime to lock the folder at | [optional]
**unlock_at** | Option<**String**> | The datetime to unlock the folder at | [optional]
**locked** | Option<**bool**> | Flag the folder as locked | [optional]
**hidden** | Option<**bool**> | Flag the folder as hidden | [optional]
**position** | Option<**i32**> | Set an explicit sort position for the folder | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


