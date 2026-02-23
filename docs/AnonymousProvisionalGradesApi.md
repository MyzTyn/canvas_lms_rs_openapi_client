# \AnonymousProvisionalGradesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**anonymous_provisional_grades_status**](AnonymousProvisionalGradesApi.md#anonymous_provisional_grades_status) | **GET** /courses/{course_id}/assignments/{assignment_id}/anonymous_provisional_grades/status | Show provisional grade status for a student



## anonymous_provisional_grades_status

> anonymous_provisional_grades_status(assignment_id, course_id, anonymous_id)
Show provisional grade status for a student

Determine whether or not the student's submission needs one or more provisional grades.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**anonymous_id** | Option<**String**> | The id of the student to show the status for |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

