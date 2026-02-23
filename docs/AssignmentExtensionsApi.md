# \AssignmentExtensionsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assignment_extensions_create**](AssignmentExtensionsApi.md#assignment_extensions_create) | **POST** /courses/{course_id}/assignments/{assignment_id}/extensions | Set extensions for student assignment submissions



## assignment_extensions_create

> assignment_extensions_create(assignment_id, course_id, assignment_extensions_create_request)
Set extensions for student assignment submissions

<b>Responses</b>  * <b>200 OK</b> if the request was successful * <b>403 Forbidden</b> if you are not allowed to extend assignments for this course * <b>400 Bad Request</b> if any of the extensions are invalid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_extensions_create_request** | Option<[**AssignmentExtensionsCreateRequest**](AssignmentExtensionsCreateRequest.md)> | Request body parameters  **Example Request:** ``` {   \"assignment_extensions\": [{     \"user_id\": 3,     \"extra_attempts\": 2   },{     \"user_id\": 2,     \"extra_attempts\": 2   }] } ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

