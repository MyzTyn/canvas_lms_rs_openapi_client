# UsageRightsSetUsageRightsForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_ids** | **Vec<String>** | [Required] List of ids of files to set usage rights for. | 
**folder_ids** | Option<**Vec<String>**> | [Optional] List of ids of folders to search for files to set usage rights for. Note that new files uploaded to these folders do not automatically inherit these rights. | [optional]
**publish** | Option<**bool**> | Whether the file(s) or folder(s) should be published on save, provided that usage rights have been specified (set to `true` to publish on save). | [optional]
**usage_rights** | [**models::UsageRightsSetUsageRightsForCoursesRequestUsageRights**](UsageRightsSetUsageRightsForCoursesRequestUsageRights.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


