# \SubmissionsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submissions_create_for_courses**](SubmissionsApi.md#submissions_create_for_courses) | **POST** /courses/{course_id}/assignments/{assignment_id}/submissions | Submit an assignment
[**submissions_create_other**](SubmissionsApi.md#submissions_create_other) | **POST** /sections/{section_id}/assignments/{assignment_id}/submissions | Submit an assignment



## submissions_create_for_courses

> submissions_create_for_courses(assignment_id, course_id, submissions_create_for_courses_request)
Submit an assignment

Make a submission for an assignment. You must be actively enrolled as a student in the course/section to do this. Concluded and pending enrollments are not permitted.  All online turn-in submission types are supported in this API. However, there are a few things that are not yet supported:  * Files can be submitted based on a file ID of a user or group file or through the {api:SubmissionsApiController#create_file file upload API}. However, there is no API yet for listing the user and group files. * Media comments can be submitted, however, there is no API yet for creating a media comment to submit. * Integration with Google Docs is not yet supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**submissions_create_for_courses_request** | Option<[**SubmissionsCreateForCoursesRequest**](SubmissionsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_create_other

> submissions_create_other(section_id, assignment_id, submissions_create_for_courses_request)
Submit an assignment

Make a submission for an assignment. You must be actively enrolled as a student in the course/section to do this. Concluded and pending enrollments are not permitted.  All online turn-in submission types are supported in this API. However, there are a few things that are not yet supported:  * Files can be submitted based on a file ID of a user or group file or through the {api:SubmissionsApiController#create_file file upload API}. However, there is no API yet for listing the user and group files. * Media comments can be submitted, however, there is no API yet for creating a media comment to submit. * Integration with Google Docs is not yet supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**submissions_create_for_courses_request** | Option<[**SubmissionsCreateForCoursesRequest**](SubmissionsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

