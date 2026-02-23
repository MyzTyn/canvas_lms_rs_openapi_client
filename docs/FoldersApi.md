# \FoldersApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**folders_api_destroy**](FoldersApi.md#folders_api_destroy) | **DELETE** /folders/{id} | Delete folder
[**folders_api_index**](FoldersApi.md#folders_api_index) | **GET** /folders/{folder_id}/folders | List folders
[**folders_copy_file**](FoldersApi.md#folders_copy_file) | **POST** /folders/{dest_folder_id}/copy_file | Copy a file
[**folders_copy_folder**](FoldersApi.md#folders_copy_folder) | **POST** /folders/{dest_folder_id}/copy_folder | Copy a folder
[**folders_create_file**](FoldersApi.md#folders_create_file) | **POST** /folders/{folder_id}/files | Upload a file
[**folders_create_for_accounts**](FoldersApi.md#folders_create_for_accounts) | **POST** /accounts/{account_id}/folders | Create folder
[**folders_create_for_courses**](FoldersApi.md#folders_create_for_courses) | **POST** /courses/{course_id}/folders | Create folder
[**folders_create_for_groups**](FoldersApi.md#folders_create_for_groups) | **POST** /groups/{group_id}/folders | Create folder
[**folders_create_for_users**](FoldersApi.md#folders_create_for_users) | **POST** /users/{user_id}/folders | Create folder
[**folders_create_other**](FoldersApi.md#folders_create_other) | **POST** /folders/{folder_id}/folders | Create folder
[**folders_list_all_folders_for_courses**](FoldersApi.md#folders_list_all_folders_for_courses) | **GET** /courses/{course_id}/folders | List all folders
[**folders_list_all_folders_for_groups**](FoldersApi.md#folders_list_all_folders_for_groups) | **GET** /groups/{group_id}/folders | List all folders
[**folders_list_all_folders_for_users**](FoldersApi.md#folders_list_all_folders_for_users) | **GET** /users/{user_id}/folders | List all folders
[**folders_media_folder_for_courses**](FoldersApi.md#folders_media_folder_for_courses) | **GET** /courses/{course_id}/folders/media | Get uploaded media folder for user
[**folders_media_folder_for_groups**](FoldersApi.md#folders_media_folder_for_groups) | **GET** /groups/{group_id}/folders/media | Get uploaded media folder for user
[**folders_resolve_path_for_courses**](FoldersApi.md#folders_resolve_path_for_courses) | **GET** /courses/{course_id}/folders/by_path/*full_path | Resolve path
[**folders_resolve_path_for_courses2**](FoldersApi.md#folders_resolve_path_for_courses2) | **GET** /courses/{course_id}/folders/by_path | Resolve path
[**folders_resolve_path_for_groups**](FoldersApi.md#folders_resolve_path_for_groups) | **GET** /groups/{group_id}/folders/by_path/*full_path | Resolve path
[**folders_resolve_path_for_groups2**](FoldersApi.md#folders_resolve_path_for_groups2) | **GET** /groups/{group_id}/folders/by_path | Resolve path
[**folders_resolve_path_for_users**](FoldersApi.md#folders_resolve_path_for_users) | **GET** /users/{user_id}/folders/by_path/*full_path | Resolve path
[**folders_resolve_path_for_users2**](FoldersApi.md#folders_resolve_path_for_users2) | **GET** /users/{user_id}/folders/by_path | Resolve path
[**folders_show_for_courses**](FoldersApi.md#folders_show_for_courses) | **GET** /courses/{course_id}/folders/{id} | Get folder
[**folders_show_for_groups**](FoldersApi.md#folders_show_for_groups) | **GET** /groups/{group_id}/folders/{id} | Get folder
[**folders_show_for_users**](FoldersApi.md#folders_show_for_users) | **GET** /users/{user_id}/folders/{id} | Get folder
[**folders_show_other**](FoldersApi.md#folders_show_other) | **GET** /folders/{id} | Get folder
[**folders_update**](FoldersApi.md#folders_update) | **PUT** /folders/{id} | Update folder



## folders_api_destroy

> folders_api_destroy(id, force)
Delete folder

Remove the specified folder. You can only delete empty folders unless you set the 'force' flag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**force** | Option<**bool**> | Set to 'true' to allow deleting a non-empty folder |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_api_index

> models::Folder folders_api_index(folder_id, page, per_page)
List folders

Returns the paginated list of folders in the folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Scope response to id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_copy_file

> String folders_copy_file(dest_folder_id, folders_copy_file_request)
Copy a file

Copy a file from elsewhere in Canvas into a folder.  Copying a file across contexts (between courses and users) is permitted, but the source and destination must belong to the same institution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dest_folder_id** | **String** | Scope response to dest_folder_id | [required] |
**folders_copy_file_request** | Option<[**FoldersCopyFileRequest**](FoldersCopyFileRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/folders/123/copy_file' \\      -H 'Authorization: Bearer <token>'      -F 'source_file_id=456' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_copy_folder

> String folders_copy_folder(dest_folder_id, folders_copy_folder_request)
Copy a folder

Copy a folder (and its contents) from elsewhere in Canvas into a folder.  Copying a folder across contexts (between courses and users) is permitted, but the source and destination must belong to the same institution. If the source and destination folders are in the same context, the source folder may not contain the destination folder. A folder will be renamed at its destination if another folder with the same name already exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dest_folder_id** | **String** | Scope response to dest_folder_id | [required] |
**folders_copy_folder_request** | Option<[**FoldersCopyFolderRequest**](FoldersCopyFolderRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/folders/123/copy_folder' \\      -H 'Authorization: Bearer <token>'      -F 'source_file_id=789' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_file

> folders_create_file(folder_id)
Upload a file

Upload a file to a folder.  This API endpoint is the first step in uploading a file. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  Only those with the \"Manage Files\" permission on a course or group can upload files to a folder in that course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Scope response to folder_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_for_accounts

> String folders_create_for_accounts(account_id, folders_create_for_courses_request)
Create folder

Creates a folder in the specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**folders_create_for_courses_request** | Option<[**FoldersCreateForCoursesRequest**](FoldersCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_for_courses

> String folders_create_for_courses(course_id, folders_create_for_courses_request)
Create folder

Creates a folder in the specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**folders_create_for_courses_request** | Option<[**FoldersCreateForCoursesRequest**](FoldersCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_for_groups

> String folders_create_for_groups(group_id, folders_create_for_courses_request)
Create folder

Creates a folder in the specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**folders_create_for_courses_request** | Option<[**FoldersCreateForCoursesRequest**](FoldersCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_for_users

> String folders_create_for_users(user_id, folders_create_for_courses_request)
Create folder

Creates a folder in the specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**folders_create_for_courses_request** | Option<[**FoldersCreateForCoursesRequest**](FoldersCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_other

> String folders_create_other(folder_id, folders_create_for_courses_request)
Create folder

Creates a folder in the specified context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | ID of the folder | [required] |
**folders_create_for_courses_request** | Option<[**FoldersCreateForCoursesRequest**](FoldersCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_list_all_folders_for_courses

> serde_json::Value folders_list_all_folders_for_courses(course_id, page, per_page)
List all folders

Returns the paginated list of all folders for the given context. This will be returned as a flat list containing all subfolders as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_list_all_folders_for_groups

> serde_json::Value folders_list_all_folders_for_groups(group_id, page, per_page)
List all folders

Returns the paginated list of all folders for the given context. This will be returned as a flat list containing all subfolders as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_list_all_folders_for_users

> serde_json::Value folders_list_all_folders_for_users(user_id, page, per_page)
List all folders

Returns the paginated list of all folders for the given context. This will be returned as a flat list containing all subfolders as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_media_folder_for_courses

> String folders_media_folder_for_courses(course_id)
Get uploaded media folder for user

Returns the details for a designated upload folder that the user has rights to upload to, and creates it if it doesn't exist.  If the current user does not have the permissions to manage files in the course or group, the folder will belong to the current user directly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_media_folder_for_groups

> String folders_media_folder_for_groups(group_id)
Get uploaded media folder for user

Returns the details for a designated upload folder that the user has rights to upload to, and creates it if it doesn't exist.  If the current user does not have the permissions to manage files in the course or group, the folder will belong to the current user directly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_courses

> serde_json::Value folders_resolve_path_for_courses(course_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_courses2

> serde_json::Value folders_resolve_path_for_courses2(course_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_groups

> serde_json::Value folders_resolve_path_for_groups(group_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_groups2

> serde_json::Value folders_resolve_path_for_groups2(group_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_users

> serde_json::Value folders_resolve_path_for_users(user_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_resolve_path_for_users2

> serde_json::Value folders_resolve_path_for_users2(user_id)
Resolve path

Given the full path to a folder, returns a list of all Folders in the path hierarchy, starting at the root folder, and ending at the requested folder. The given path is relative to the context's root folder and does not include the root folder's name (e.g., \"course files\"). If an empty path is given, the context's root folder alone is returned. Otherwise, if no folder exists with the given full path, a Not Found error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_show_for_courses

> String folders_show_for_courses(course_id, id)
Get folder

Returns the details for a folder  You can get the root folder from a context by using 'root' as the :id. For example, you could get the root folder for a course like:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_show_for_groups

> String folders_show_for_groups(group_id, id)
Get folder

Returns the details for a folder  You can get the root folder from a context by using 'root' as the :id. For example, you could get the root folder for a course like:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_show_for_users

> String folders_show_for_users(user_id, id)
Get folder

Returns the details for a folder  You can get the root folder from a context by using 'root' as the :id. For example, you could get the root folder for a course like:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_show_other

> String folders_show_other(id)
Get folder

Returns the details for a folder  You can get the root folder from a context by using 'root' as the :id. For example, you could get the root folder for a course like:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_update

> String folders_update(id, folders_update_request)
Update folder

Updates a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**folders_update_request** | Option<[**FoldersUpdateRequest**](FoldersUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl -XPUT 'https://<canvas>/api/v1/folders/<folder_id>' \\      -F 'name=<new_name>' \\      -F 'locked=true' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

