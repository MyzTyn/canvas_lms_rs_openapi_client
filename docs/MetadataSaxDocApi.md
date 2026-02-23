# \MetadataSaxDocApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metadata_sax_doc_api_index_for_courses**](MetadataSaxDocApi.md#metadata_sax_doc_api_index_for_courses) | **GET** /courses/{course_id}/files | List files
[**metadata_sax_doc_api_index_for_groups**](MetadataSaxDocApi.md#metadata_sax_doc_api_index_for_groups) | **GET** /groups/{group_id}/files | List files
[**metadata_sax_doc_api_index_for_users**](MetadataSaxDocApi.md#metadata_sax_doc_api_index_for_users) | **GET** /users/{user_id}/files | List files
[**metadata_sax_doc_api_index_other**](MetadataSaxDocApi.md#metadata_sax_doc_api_index_other) | **GET** /folders/{folder_id}/files | List files
[**metadata_sax_doc_api_quota_for_courses**](MetadataSaxDocApi.md#metadata_sax_doc_api_quota_for_courses) | **GET** /courses/{course_id}/files/quota | Get quota information
[**metadata_sax_doc_api_quota_for_groups**](MetadataSaxDocApi.md#metadata_sax_doc_api_quota_for_groups) | **GET** /groups/{group_id}/files/quota | Get quota information
[**metadata_sax_doc_api_quota_for_users**](MetadataSaxDocApi.md#metadata_sax_doc_api_quota_for_users) | **GET** /users/{user_id}/files/quota | Get quota information
[**metadata_sax_doc_api_show_for_courses**](MetadataSaxDocApi.md#metadata_sax_doc_api_show_for_courses) | **GET** /courses/{course_id}/files/{id} | Get file
[**metadata_sax_doc_api_show_for_groups**](MetadataSaxDocApi.md#metadata_sax_doc_api_show_for_groups) | **GET** /groups/{group_id}/files/{id} | Get file
[**metadata_sax_doc_api_show_for_users**](MetadataSaxDocApi.md#metadata_sax_doc_api_show_for_users) | **GET** /users/{user_id}/files/{id} | Get file
[**metadata_sax_doc_api_show_other**](MetadataSaxDocApi.md#metadata_sax_doc_api_show_other) | **GET** /files/{id} | Get file
[**metadata_sax_doc_api_update**](MetadataSaxDocApi.md#metadata_sax_doc_api_update) | **PUT** /files/{id} | Update file
[**metadata_sax_doc_destroy**](MetadataSaxDocApi.md#metadata_sax_doc_destroy) | **DELETE** /files/{id} | Delete file
[**metadata_sax_doc_file_ref**](MetadataSaxDocApi.md#metadata_sax_doc_file_ref) | **GET** /courses/{course_id}/files/file_ref/{migration_id} | Translate file reference
[**metadata_sax_doc_icon_metadata**](MetadataSaxDocApi.md#metadata_sax_doc_icon_metadata) | **GET** /files/{id}/icon_metadata | Get icon metadata
[**metadata_sax_doc_public_url**](MetadataSaxDocApi.md#metadata_sax_doc_public_url) | **GET** /files/{id}/public_url | Get public inline preview url
[**metadata_sax_doc_reset_verifier**](MetadataSaxDocApi.md#metadata_sax_doc_reset_verifier) | **POST** /files/{id}/reset_verifier | Reset link verifier



## metadata_sax_doc_api_index_for_courses

> serde_json::Value metadata_sax_doc_api_index_for_courses(course_id, content_types, exclude_content_types, search_term, include, only, sort, order, page, per_page)
List files

Returns the paginated list of files for the folder or course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**content_types** | Option<[**Vec<String>**](String.md)> | [String] Filter results by content-type. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**exclude_content_types** | Option<[**Vec<String>**](String.md)> | [String] Exclude given content-types from your results. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**search_term** | Option<**String**> | The partial name of the files to match and return. |  |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**only** | Option<[**Vec<String>**](String.md)> | [Array] Array of information to restrict to. Overrides include[]  \"names\":: only returns file name information |  |
**sort** | Option<**String**> | Sort results by this field. Defaults to 'name'. Note that `sort=user` implies `include[]=user`. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
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


## metadata_sax_doc_api_index_for_groups

> serde_json::Value metadata_sax_doc_api_index_for_groups(group_id, content_types, exclude_content_types, search_term, include, only, sort, order, page, per_page)
List files

Returns the paginated list of files for the folder or course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_types** | Option<[**Vec<String>**](String.md)> | [String] Filter results by content-type. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**exclude_content_types** | Option<[**Vec<String>**](String.md)> | [String] Exclude given content-types from your results. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**search_term** | Option<**String**> | The partial name of the files to match and return. |  |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**only** | Option<[**Vec<String>**](String.md)> | [Array] Array of information to restrict to. Overrides include[]  \"names\":: only returns file name information |  |
**sort** | Option<**String**> | Sort results by this field. Defaults to 'name'. Note that `sort=user` implies `include[]=user`. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
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


## metadata_sax_doc_api_index_for_users

> serde_json::Value metadata_sax_doc_api_index_for_users(user_id, content_types, exclude_content_types, search_term, include, only, sort, order, page, per_page)
List files

Returns the paginated list of files for the folder or course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_types** | Option<[**Vec<String>**](String.md)> | [String] Filter results by content-type. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**exclude_content_types** | Option<[**Vec<String>**](String.md)> | [String] Exclude given content-types from your results. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**search_term** | Option<**String**> | The partial name of the files to match and return. |  |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**only** | Option<[**Vec<String>**](String.md)> | [Array] Array of information to restrict to. Overrides include[]  \"names\":: only returns file name information |  |
**sort** | Option<**String**> | Sort results by this field. Defaults to 'name'. Note that `sort=user` implies `include[]=user`. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
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


## metadata_sax_doc_api_index_other

> serde_json::Value metadata_sax_doc_api_index_other(folder_id, content_types, exclude_content_types, search_term, include, only, sort, order, page, per_page)
List files

Returns the paginated list of files for the folder or course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | ID of the folder | [required] |
**content_types** | Option<[**Vec<String>**](String.md)> | [String] Filter results by content-type. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**exclude_content_types** | Option<[**Vec<String>**](String.md)> | [String] Exclude given content-types from your results. You can specify type/subtype pairs (e.g., 'image/jpeg'), or simply types (e.g., 'image', which will match 'image/gif', 'image/jpeg', etc.). |  |
**search_term** | Option<**String**> | The partial name of the files to match and return. |  |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**only** | Option<[**Vec<String>**](String.md)> | [Array] Array of information to restrict to. Overrides include[]  \"names\":: only returns file name information |  |
**sort** | Option<**String**> | Sort results by this field. Defaults to 'name'. Note that `sort=user` implies `include[]=user`. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
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


## metadata_sax_doc_api_quota_for_courses

> metadata_sax_doc_api_quota_for_courses(course_id)
Get quota information

Returns the total and used storage quota for the course, group, or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_quota_for_groups

> metadata_sax_doc_api_quota_for_groups(group_id)
Get quota information

Returns the total and used storage quota for the course, group, or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_quota_for_users

> metadata_sax_doc_api_quota_for_users(user_id)
Get quota information

Returns the total and used storage quota for the course, group, or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_show_for_courses

> String metadata_sax_doc_api_show_for_courses(course_id, id, include, replacement_chain_context_type, replacement_chain_context_id)
Get file

Returns the standard attachment json object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**replacement_chain_context_type** | Option<**String**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Must be set to 'course' or 'account'. The \"replacement_chain_context_id\" parameter must also be included. |  |
**replacement_chain_context_id** | Option<**i32**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Indicates the context ID Canvas should use when following the \"replacement chain.\" The \"replacement_chain_context_type\" parameter must also be included. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_show_for_groups

> String metadata_sax_doc_api_show_for_groups(group_id, id, include, replacement_chain_context_type, replacement_chain_context_id)
Get file

Returns the standard attachment json object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**replacement_chain_context_type** | Option<**String**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Must be set to 'course' or 'account'. The \"replacement_chain_context_id\" parameter must also be included. |  |
**replacement_chain_context_id** | Option<**i32**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Indicates the context ID Canvas should use when following the \"replacement chain.\" The \"replacement_chain_context_type\" parameter must also be included. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_show_for_users

> String metadata_sax_doc_api_show_for_users(user_id, id, include, replacement_chain_context_type, replacement_chain_context_id)
Get file

Returns the standard attachment json object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**replacement_chain_context_type** | Option<**String**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Must be set to 'course' or 'account'. The \"replacement_chain_context_id\" parameter must also be included. |  |
**replacement_chain_context_id** | Option<**i32**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Indicates the context ID Canvas should use when following the \"replacement chain.\" The \"replacement_chain_context_type\" parameter must also be included. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_show_other

> String metadata_sax_doc_api_show_other(id, include, replacement_chain_context_type, replacement_chain_context_id)
Get file

Returns the standard attachment json object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"user\"] Array of additional information to include.  \"user\":: the user who uploaded the file or last edited its content \"usage_rights\":: copyright and license information for the file (see UsageRights) |  |
**replacement_chain_context_type** | Option<**String**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Must be set to 'course' or 'account'. The \"replacement_chain_context_id\" parameter must also be included. |  |
**replacement_chain_context_id** | Option<**i32**> | When a user replaces a file during upload, Canvas keeps track of the \"replacement chain.\"  Include this parameter if you wish Canvas to follow the replacement chain if the requested file was deleted and replaced by another.  Indicates the context ID Canvas should use when following the \"replacement chain.\" The \"replacement_chain_context_type\" parameter must also be included. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_api_update

> String metadata_sax_doc_api_update(id, metadata_sax_doc_api_update_request)
Update file

Update some settings on the specified file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**metadata_sax_doc_api_update_request** | Option<[**MetadataSaxDocApiUpdateRequest**](MetadataSaxDocApiUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_destroy

> String metadata_sax_doc_destroy(id, replace)
Delete file

Remove the specified file. Unlike most other DELETE endpoints, using this endpoint will result in comprehensive, irretrievable destruction of the file. It should be used with the `replace` parameter set to true in cases where the file preview also needs to be destroyed (such as to remove files that violate privacy laws).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**replace** | Option<**bool**> | This action is irreversible. If replace is set to true the file contents will be replaced with a generic \"file has been removed\" file. This also destroys any previews that have been generated for the file. Must have manage files and become other users permissions |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_file_ref

> String metadata_sax_doc_file_ref(course_id, migration_id)
Translate file reference

Get information about a file from a course copy file reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**migration_id** | **String** | Scope response to migration_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_icon_metadata

> metadata_sax_doc_icon_metadata(id)
Get icon metadata

Returns the icon maker file attachment metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_public_url

> metadata_sax_doc_public_url(id, submission_id)
Get public inline preview url

Determine the URL that should be used for inline preview of the file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**submission_id** | Option<**i32**> | The id of the submission the file is associated with.  Provide this argument to gain access to a file that has been submitted to an assignment (Canvas will verify that the file belongs to the submission and the calling user has rights to view the submission). |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_sax_doc_reset_verifier

> String metadata_sax_doc_reset_verifier(id)
Reset link verifier

Resets the link verifier. Any existing links to the file using the previous hard-coded \"verifier\" parameter will no longer automatically grant access.  Must have manage files and become other users permissions

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

