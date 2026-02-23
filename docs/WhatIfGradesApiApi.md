# \WhatIfGradesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**what_if_grades_api_reset_for_student_course**](WhatIfGradesApiApi.md#what_if_grades_api_reset_for_student_course) | **PUT** /courses/{course_id}/what_if_grades/reset | Reset the what-if scores for the current user for an entire course and recalculate grades
[**what_if_grades_api_update**](WhatIfGradesApiApi.md#what_if_grades_api_update) | **PUT** /submissions/{id}/what_if_grades | Update a submission's what-if score and calculate grades



## what_if_grades_api_reset_for_student_course

> serde_json::Value what_if_grades_api_reset_for_student_course(course_id)
Reset the what-if scores for the current user for an entire course and recalculate grades

Resets all what-if scores for a student in a course and recalculates grades.

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


## what_if_grades_api_update

> serde_json::Value what_if_grades_api_update(id, what_if_grades_api_update_request)
Update a submission's what-if score and calculate grades

Enter a what if score for a submission and receive the calculated grades Grade calculation is a costly operation, so this API should be used sparingly

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**what_if_grades_api_update_request** | Option<[**WhatIfGradesApiUpdateRequest**](WhatIfGradesApiUpdateRequest.md)> | Request body parameters |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

