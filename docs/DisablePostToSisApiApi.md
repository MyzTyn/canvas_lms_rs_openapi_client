# \DisablePostToSisApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_post_to_sis_api_disable_post_to_sis**](DisablePostToSisApiApi.md#disable_post_to_sis_api_disable_post_to_sis) | **PUT** /courses/{course_id}/disable_post_to_sis | Disable assignments currently enabled for grade export to SIS



## disable_post_to_sis_api_disable_post_to_sis

> disable_post_to_sis_api_disable_post_to_sis(course_id, disable_post_to_sis_api_disable_post_to_sis_request)
Disable assignments currently enabled for grade export to SIS

Disable all assignments flagged as \"post_to_sis\", with the option of making it specific to a grading period, in a course.  On success, the response will be 204 No Content with an empty body.  On failure, the response will be 400 Bad Request with a body of a specific message.  For disabling assignments in a specific grading period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The ID of the course. | [required] |
**disable_post_to_sis_api_disable_post_to_sis_request** | Option<[**DisablePostToSisApiDisablePostToSisRequest**](DisablePostToSisApiDisablePostToSisRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/sis/courses/<course_id>/disable_post_to_sis' \\      -X PUT \\      -H \"Authorization: Bearer <token>\" \\      -H \"Content-Length: 0\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

