# \UsageRightsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**usage_rights_licenses_for_courses**](UsageRightsApi.md#usage_rights_licenses_for_courses) | **GET** /courses/{course_id}/content_licenses | List licenses
[**usage_rights_licenses_for_groups**](UsageRightsApi.md#usage_rights_licenses_for_groups) | **GET** /groups/{group_id}/content_licenses | List licenses
[**usage_rights_licenses_for_users**](UsageRightsApi.md#usage_rights_licenses_for_users) | **GET** /users/{user_id}/content_licenses | List licenses
[**usage_rights_remove_usage_rights_for_courses**](UsageRightsApi.md#usage_rights_remove_usage_rights_for_courses) | **DELETE** /courses/{course_id}/usage_rights | Remove usage rights
[**usage_rights_remove_usage_rights_for_groups**](UsageRightsApi.md#usage_rights_remove_usage_rights_for_groups) | **DELETE** /groups/{group_id}/usage_rights | Remove usage rights
[**usage_rights_remove_usage_rights_for_users**](UsageRightsApi.md#usage_rights_remove_usage_rights_for_users) | **DELETE** /users/{user_id}/usage_rights | Remove usage rights
[**usage_rights_set_usage_rights_for_courses**](UsageRightsApi.md#usage_rights_set_usage_rights_for_courses) | **PUT** /courses/{course_id}/usage_rights | Set usage rights
[**usage_rights_set_usage_rights_for_groups**](UsageRightsApi.md#usage_rights_set_usage_rights_for_groups) | **PUT** /groups/{group_id}/usage_rights | Set usage rights
[**usage_rights_set_usage_rights_for_users**](UsageRightsApi.md#usage_rights_set_usage_rights_for_users) | **PUT** /users/{user_id}/usage_rights | Set usage rights



## usage_rights_licenses_for_courses

> serde_json::Value usage_rights_licenses_for_courses(course_id)
List licenses

A paginated list of licenses that can be applied

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


## usage_rights_licenses_for_groups

> serde_json::Value usage_rights_licenses_for_groups(group_id)
List licenses

A paginated list of licenses that can be applied

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


## usage_rights_licenses_for_users

> serde_json::Value usage_rights_licenses_for_users(user_id)
List licenses

A paginated list of licenses that can be applied

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


## usage_rights_remove_usage_rights_for_courses

> usage_rights_remove_usage_rights_for_courses(course_id, file_ids, folder_ids)
Remove usage rights

Removes copyright and license information associated with one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**file_ids** | [**Vec<String>**](String.md) | [Required] List of ids of files to remove associated usage rights from. | [required] |
**folder_ids** | Option<[**Vec<String>**](String.md)> | [Optional] List of ids of folders. Usage rights will be removed from all files in these folders. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usage_rights_remove_usage_rights_for_groups

> usage_rights_remove_usage_rights_for_groups(group_id, file_ids, folder_ids)
Remove usage rights

Removes copyright and license information associated with one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**file_ids** | [**Vec<String>**](String.md) | [Required] List of ids of files to remove associated usage rights from. | [required] |
**folder_ids** | Option<[**Vec<String>**](String.md)> | [Optional] List of ids of folders. Usage rights will be removed from all files in these folders. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usage_rights_remove_usage_rights_for_users

> usage_rights_remove_usage_rights_for_users(user_id, file_ids, folder_ids)
Remove usage rights

Removes copyright and license information associated with one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**file_ids** | [**Vec<String>**](String.md) | [Required] List of ids of files to remove associated usage rights from. | [required] |
**folder_ids** | Option<[**Vec<String>**](String.md)> | [Optional] List of ids of folders. Usage rights will be removed from all files in these folders. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usage_rights_set_usage_rights_for_courses

> String usage_rights_set_usage_rights_for_courses(course_id, usage_rights_set_usage_rights_for_courses_request)
Set usage rights

Sets copyright and license information for one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**usage_rights_set_usage_rights_for_courses_request** | Option<[**UsageRightsSetUsageRightsForCoursesRequest**](UsageRightsSetUsageRightsForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usage_rights_set_usage_rights_for_groups

> String usage_rights_set_usage_rights_for_groups(group_id, usage_rights_set_usage_rights_for_courses_request)
Set usage rights

Sets copyright and license information for one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**usage_rights_set_usage_rights_for_courses_request** | Option<[**UsageRightsSetUsageRightsForCoursesRequest**](UsageRightsSetUsageRightsForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usage_rights_set_usage_rights_for_users

> String usage_rights_set_usage_rights_for_users(user_id, usage_rights_set_usage_rights_for_courses_request)
Set usage rights

Sets copyright and license information for one or more files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**usage_rights_set_usage_rights_for_courses_request** | Option<[**UsageRightsSetUsageRightsForCoursesRequest**](UsageRightsSetUsageRightsForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

