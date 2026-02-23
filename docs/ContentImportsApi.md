# \ContentImportsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_imports_copy_course_content**](ContentImportsApi.md#content_imports_copy_course_content) | **POST** /courses/{course_id}/course_copy | Copy course content
[**content_imports_copy_course_status**](ContentImportsApi.md#content_imports_copy_course_status) | **GET** /courses/{course_id}/course_copy/{id} | Get course copy status



## content_imports_copy_course_content

> content_imports_copy_course_content(course_id, content_imports_copy_course_content_request)
Copy course content

DEPRECATED: Please use the {api:ContentMigrationsController#create Content Migrations API}  Copies content from one course into another. The default is to copy all course content. You can control specific types to copy by using either the 'except' option or the 'only' option.  The response is the same as the course copy status endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**content_imports_copy_course_content_request** | Option<[**ContentImportsCopyCourseContentRequest**](ContentImportsCopyCourseContentRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_imports_copy_course_status

> content_imports_copy_course_status(course_id, id)
Get course copy status

DEPRECATED: Please use the {api:ContentMigrationsController#create Content Migrations API}  Retrieve the status of a course copy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

