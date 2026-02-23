# \ContentExportsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_exports_api_create_for_courses**](ContentExportsApiApi.md#content_exports_api_create_for_courses) | **POST** /courses/{course_id}/content_exports | Export content
[**content_exports_api_create_for_groups**](ContentExportsApiApi.md#content_exports_api_create_for_groups) | **POST** /groups/{group_id}/content_exports | Export content
[**content_exports_api_create_for_users**](ContentExportsApiApi.md#content_exports_api_create_for_users) | **POST** /users/{user_id}/content_exports | Export content
[**content_exports_api_index_for_courses**](ContentExportsApiApi.md#content_exports_api_index_for_courses) | **GET** /courses/{course_id}/content_exports | List content exports
[**content_exports_api_index_for_groups**](ContentExportsApiApi.md#content_exports_api_index_for_groups) | **GET** /groups/{group_id}/content_exports | List content exports
[**content_exports_api_index_for_users**](ContentExportsApiApi.md#content_exports_api_index_for_users) | **GET** /users/{user_id}/content_exports | List content exports
[**content_exports_api_show_for_courses**](ContentExportsApiApi.md#content_exports_api_show_for_courses) | **GET** /courses/{course_id}/content_exports/{id} | Show content export
[**content_exports_api_show_for_groups**](ContentExportsApiApi.md#content_exports_api_show_for_groups) | **GET** /groups/{group_id}/content_exports/{id} | Show content export
[**content_exports_api_show_for_users**](ContentExportsApiApi.md#content_exports_api_show_for_users) | **GET** /users/{user_id}/content_exports/{id} | Show content export



## content_exports_api_create_for_courses

> String content_exports_api_create_for_courses(course_id, content_exports_api_create_for_courses_request)
Export content

Begin a content export job for a course, group, or user.  You can use the {api:ProgressController#show Progress API} to track the progress of the export. The migration's progress is linked to with the _progress_url_ value.  When the export completes, use the {api:ContentExportsApiController#show Show content export} endpoint to retrieve a download URL for the exported content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**content_exports_api_create_for_courses_request** | Option<[**ContentExportsApiCreateForCoursesRequest**](ContentExportsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_exports_api_create_for_groups

> String content_exports_api_create_for_groups(group_id, content_exports_api_create_for_courses_request)
Export content

Begin a content export job for a course, group, or user.  You can use the {api:ProgressController#show Progress API} to track the progress of the export. The migration's progress is linked to with the _progress_url_ value.  When the export completes, use the {api:ContentExportsApiController#show Show content export} endpoint to retrieve a download URL for the exported content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_exports_api_create_for_courses_request** | Option<[**ContentExportsApiCreateForCoursesRequest**](ContentExportsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_exports_api_create_for_users

> String content_exports_api_create_for_users(user_id, content_exports_api_create_for_courses_request)
Export content

Begin a content export job for a course, group, or user.  You can use the {api:ProgressController#show Progress API} to track the progress of the export. The migration's progress is linked to with the _progress_url_ value.  When the export completes, use the {api:ContentExportsApiController#show Show content export} endpoint to retrieve a download URL for the exported content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_exports_api_create_for_courses_request** | Option<[**ContentExportsApiCreateForCoursesRequest**](ContentExportsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_exports_api_index_for_courses

> serde_json::Value content_exports_api_index_for_courses(course_id, page, per_page)
List content exports

A paginated list of the past and pending content export jobs for a course, group, or user. Exports are returned newest first.

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


## content_exports_api_index_for_groups

> serde_json::Value content_exports_api_index_for_groups(group_id, page, per_page)
List content exports

A paginated list of the past and pending content export jobs for a course, group, or user. Exports are returned newest first.

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


## content_exports_api_index_for_users

> serde_json::Value content_exports_api_index_for_users(user_id, page, per_page)
List content exports

A paginated list of the past and pending content export jobs for a course, group, or user. Exports are returned newest first.

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


## content_exports_api_show_for_courses

> String content_exports_api_show_for_courses(course_id, id)
Show content export

Get information about a single content export.

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


## content_exports_api_show_for_groups

> String content_exports_api_show_for_groups(group_id, id)
Show content export

Get information about a single content export.

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


## content_exports_api_show_for_users

> String content_exports_api_show_for_users(user_id, id)
Show content export

Get information about a single content export.

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

